use super::{LspServer, Transport};

#[derive(Debug)]
pub(super) struct BrowserServer {}

impl BrowserServer {
    pub(super) fn new() -> Self { Self {} }

    pub(super) async fn run<T>(self, transport: T) -> anyhow::Result<()>
    where
        T: Transport
    {
        transport.close()?;
        Ok(())
    }
}

use lsp_types::{ClientCapabilities, ServerCapabilities};

#[async_trait]
impl LspServer for BrowserServer {
    type Notification = i32;
    type Request = i32;
    type Response = i32;
    type InitializeParams = lsp_types::InitializeParams;
    type InitializeResult = lsp_types::InitializeResult;
    type Error = anyhow::Error;

    async fn handle_notification(&mut self, msg: Self::Notification) -> Result<(), Self::Error> {
        todo!()
    }

    async fn handle_request(&mut self, msg: Self::Request) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn initialize(
        &mut self,
        params: Self::InitializeParams
    ) -> Result<Self::InitializeResult, Self::Error> {
        let server_capabilities = server_capabilities(&params.capabilities);

        let initialize_result = lsp_types::InitializeResult {
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

// web-browser-lsp://
// first version of buffer spec
// ```
// {url}
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
