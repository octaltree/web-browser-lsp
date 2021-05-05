use super::{ResponseHandler, Worker};
use crate::{lsp_ext, LspServer, Transport};
use lsp_server::{ErrorCode, RequestId};
use lsp_types::{
    notification::Notification as _, request::Request as _, ClientCapabilities, InitializeParams,
    InitializeResult, Position, Range, ServerCapabilities, TextEdit
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, future::Future, panic};

#[derive(Debug)]
struct ErrorBody {
    code: ErrorCode,
    message: String
}

fn _impl_lsp_error() {
    impl ErrorBody {
        fn new(code: ErrorCode, message: String) -> ErrorBody { ErrorBody { code, message } }

        fn into_response(self, id: RequestId) -> lsp_server::Response {
            lsp_server::Response::new_err(id, self.code as i32, self.message)
        }
    }

    impl fmt::Display for ErrorBody {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Language Server request failed with {}. ({})",
                self.code as i32, self.message
            )
        }
    }

    impl std::error::Error for ErrorBody {}
}

#[async_trait]
impl<T> LspServer for Worker<T>
where
    T: Transport<Message = lsp_server::Message> + Send
{
    type Notification = lsp_server::Notification;
    type Request = lsp_server::Request;
    type Response = lsp_server::Response;
    type InitializeParams = InitializeParams;
    type InitializeResult = InitializeResult;

    async fn handle_notification(
        &mut self,
        msg: Self::Notification
    ) -> Result<bool, anyhow::Error> {
        match msg.method.as_str() {
            lsp_types::notification::Exit::METHOD => return Ok(true),
            lsp_types::notification::DidChangeConfiguration::METHOD => {
                self.send_request::<lsp_types::request::WorkspaceConfiguration>(
                    lsp_types::ConfigurationParams {
                        items: vec![lsp_types::ConfigurationItem {
                            scope_uri: None,
                            section: Some("web-browser-lsp".to_string())
                        }]
                    },
                    |worker, resp| {
                        log::debug!("config update response: '{:?}", resp);
                        let Self::Response { error, result, .. } = resp;
                        match (error, result) {
                            (Some(err), _) => {
                                log::error!("failed to fetch the server settings: {:?}", err)
                            }
                            (None, Some(mut configs)) => {
                                if let Some(json) = configs.get_mut(0) {
                                    // TODO
                                }
                            }
                            (None, None) => log::error!(
                                "received empty server settings response from the client"
                            )
                        }
                        Ok(())
                    }
                )?;
            }
            lsp_types::notification::DidChangeTextDocument::METHOD => {
                let params =
                    parse_notification::<lsp_types::notification::DidChangeTextDocument>(msg)?;
                // NOTE: setting full sync
                let first = params.content_changes.into_iter().next();
                if let Some(changed) = first {
                    self.content = changed.text;
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn handle_request(
        &mut self,
        msg: Self::Request
    ) -> Result<Self::Response, anyhow::Error> {
        if self.shutdown_requested {
            let response = ErrorBody::new(
                ErrorCode::InvalidRequest,
                "Shutdown already requested.".into()
            )
            .into_response(msg.id);
            return Ok(response);
        }
        let response = match msg.method.as_str() {
            lsp_types::request::Shutdown::METHOD => {
                self.lift_request::<lsp_types::request::Shutdown, _>(msg, |worker, ()| async move {
                    worker.shutdown_requested = true;
                    Ok(())
                })
                .await?
            }
            lsp_ext::Tab::METHOD => {
                self.lift_request::<lsp_ext::Tab, _>(msg, |worker, params| async move {
                    let browser = worker
                        .playwright
                        .chromium()
                        .launcher()
                        .headless(false)
                        .launch()
                        .await?;
                    let context = browser.context_builder().build().await?;
                    let page = context.new_page().await?;
                    worker.active_tab = Some(page);
                    Ok(())
                })
                .await?
            }
            lsp_types::request::Formatting::METHOD => {
                self.lift_request::<lsp_types::request::Formatting, _>(
                    msg,
                    |worker, _params| async move {
                        // FIXME: count char
                        if let Some(tab) = &worker.active_tab {
                            let s = tab.inner_text("body", None).await?;
                            let n = s.lines().count();
                            let last = s.lines().last().unwrap_or_default();
                            Ok(Some(vec![TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 0,
                                        character: 0
                                    },
                                    end: Position {
                                        line: n as u32,
                                        character: last.len() as u32
                                    }
                                },
                                new_text: s
                            }]))
                        } else {
                            let n = worker.content.lines().count();
                            let last = worker.content.lines().last().unwrap_or_default();
                            Ok(Some(vec![TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 0,
                                        character: 0
                                    },
                                    end: Position {
                                        line: n as u32,
                                        character: last.len() as u32
                                    }
                                },
                                new_text: "".into()
                            }]))
                        }
                    }
                )
                .await?
            }
            _ => {
                log::error!("unknown request: {:?}", msg);
                lsp_server::Response::new_err(
                    msg.id,
                    lsp_server::ErrorCode::MethodNotFound as i32,
                    "unknown request".to_string()
                )
            }
        };
        Ok(response)
    }

    async fn initialize(
        &mut self,
        params: Self::InitializeParams
    ) -> Result<Self::InitializeResult, anyhow::Error> {
        let server_capabilities = server_capabilities(&params.capabilities);

        let initialize_result = InitializeResult {
            capabilities: server_capabilities,
            server_info: Some(lsp_types::ServerInfo {
                name: env!("CARGO_PKG_NAME").into(),
                version: Some(env!("CARGO_PKG_VERSION").into())
            }),
            offset_encoding: if supports_utf8(&params.capabilities) {
                Some("utf-8".to_string())
            } else {
                None
            }
        };
        Ok(initialize_result)
    }
}

impl<T> Worker<T>
where
    T: Transport<Message = lsp_server::Message> + Send
{
    async fn lift_request<'a, R, F>(
        &'a mut self,
        msg: lsp_server::Request,
        f: fn(&'a mut Self, R::Params) -> F
    ) -> anyhow::Result<<Self as LspServer>::Response>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
        F: Future<Output = anyhow::Result<R::Result>>
    {
        let (id, params) = match parse_request::<R>(msg) {
            Ok((id, params)) => (id, params),
            Err(response) => return Ok(response)
        };
        // rust-analyzer handles panics
        let result = f(self, params).await;
        let response = result_to_response::<R>(id, result);
        Ok(response)
    }

    fn send_request<R: lsp_types::request::Request>(
        &mut self,
        params: R::Params,
        handler: ResponseHandler<T>
    ) -> anyhow::Result<()> {
        let request =
            self.transport_queue
                .outgoing
                .register(R::METHOD.to_string(), params, handler);
        self.transport.send(request.into())?;
        Ok(())
    }
}

fn parse_notification<N>(msg: lsp_server::Notification) -> anyhow::Result<N::Params>
where
    N: lsp_types::notification::Notification + 'static,
    N::Params: DeserializeOwned + 'static
{
    let lsp_server::Notification { params, .. } = msg;
    let res: N::Params = serde_json::from_value(params)?;
    Ok(res)
}

fn parse_request<R>(
    msg: lsp_server::Request
) -> Result<(RequestId, R::Params), lsp_server::Response>
where
    R: lsp_types::request::Request + 'static,
    R::Params: DeserializeOwned + 'static
{
    let lsp_server::Request { id, params, .. } = msg;
    let res: Result<R::Params, _> = serde_json::from_value(params);
    match res {
        Ok(params) => Ok((id, params)),
        Err(err) => {
            let response =
                lsp_server::Response::new_err(id, ErrorCode::InvalidParams as i32, err.to_string());
            Err(response)
        }
    }
}

fn result_to_response<R>(
    id: lsp_server::RequestId,
    result: anyhow::Result<R::Result>
) -> lsp_server::Response
where
    R: lsp_types::request::Request + 'static,
    R::Params: DeserializeOwned + 'static,
    R::Result: Serialize + 'static
{
    match result {
        Ok(resp) => lsp_server::Response::new_ok(id, &resp),
        Err(e) => match e.downcast::<ErrorBody>() {
            Ok(e) => e.into_response(id),
            // Er(e) if is_canceled(e) => {ErrorCode::ContentModified}
            Err(e) => lsp_server::Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                e.to_string()
            )
        }
    }
}

pub fn server_capabilities(client_caps: &ClientCapabilities) -> ServerCapabilities {
    use lsp_types::{
        ExecuteCommandOptions, OneOf, TextDocumentSyncCapability, TextDocumentSyncKind,
        TextDocumentSyncOptions, WorkDoneProgressOptions
    };
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: None,
                change: Some(TextDocumentSyncKind::Full),
                will_save: None,
                will_save_wait_until: None,
                save: None
            }
        )),
        selection_range_provider: None,
        hover_provider: None,
        completion_provider: None,
        signature_help_provider: None,
        definition_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        references_provider: None,
        document_highlight_provider: None,
        document_symbol_provider: None,
        workspace_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_formatting_provider: Some(OneOf::Left(true)),
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        rename_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        experimental: None
    }
}

fn supports_utf8(caps: &lsp_types::ClientCapabilities) -> bool {
    caps.offset_encoding
        .as_deref()
        .unwrap_or_default()
        .iter()
        .any(|it| it == "utf-8")
}

// web-browser-lsp://
// first version of buffer spec
// respects CommonMark or Github Flavored but position is dynamic like browsh
// ```
// [url]
// blank line
// body
// ```
// * single tab
// * goto url
// * reload
// * text
// * click
//   - a, [onclick]
//   - [input=radio], [input=checkbox]
//   - select
// * input
//   - [type=text], textarea, [contenteditable]
