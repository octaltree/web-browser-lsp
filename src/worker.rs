use super::{LspServer, Transport};

#[derive(Debug)]
pub(super) struct Worker {}

impl Worker {
    pub(super) fn new() -> Self { Self {} }

    pub(super) async fn run<T>(mut self, transport: &mut T) -> anyhow::Result<()>
    where
        T: Transport<Message = lsp_server::Message>
    {
        loop {
            let msg = transport.next_message()?;
            if self.handle_message(transport, msg).await? {
                break;
            }
        }
        Ok(())
    }

    async fn handle_message<T>(
        &mut self,
        transport: &mut T,
        msg: lsp_server::Message
    ) -> anyhow::Result<bool>
    where
        T: Transport<Message = lsp_server::Message>
    {
        let should_close = match msg {
            lsp_server::Message::Request(req) => {
                let response = self.handle_request(req).await?;
                transport.send(lsp_server::Message::Response(response))?;
                false
            }
            lsp_server::Message::Notification(notif) => self.handle_notification(notif).await?,
            lsp_server::Message::Response(_) => unreachable!()
        };
        Ok(should_close)
    }
}

mod lsp {
    use super::Worker;
    use crate::LspServer;
    use lsp_types::{
        notification::Notification as _, ClientCapabilities, InitializeParams, InitializeResult,
        ServerCapabilities
    };

    #[async_trait]
    impl LspServer for Worker {
        type Notification = lsp_server::Notification;
        type Request = lsp_server::Request;
        type Response = lsp_server::Response;
        type InitializeParams = InitializeParams;
        type InitializeResult = InitializeResult;

        async fn handle_notification(
            &mut self,
            msg: Self::Notification
        ) -> Result<bool, anyhow::Error> {
            if msg.method == lsp_types::notification::Exit::METHOD {
                return Ok(true);
            }
            Ok(false)
        }

        async fn handle_request(
            &mut self,
            msg: Self::Request
        ) -> Result<Self::Response, anyhow::Error> {
            let response = match &msg.method {
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

    pub fn server_capabilities(client_caps: &ClientCapabilities) -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: None,
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
            document_formatting_provider: None,
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
