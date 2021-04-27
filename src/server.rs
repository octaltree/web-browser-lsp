use super::{LspServer, Transport};

#[derive(Debug)]
pub(super) struct BrowserServer {}

impl BrowserServer {
    pub(super) fn new() -> Self { Self {} }

    pub(super) async fn run<T>(self, transport: T) -> anyhow::Result<()>
    where
        T: Transport
    {
        Ok(())
    }
}

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
        todo!()
    }
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
