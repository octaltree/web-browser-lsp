#[macro_use]
extern crate async_trait;

mod server;
mod transport;

#[async_trait]
trait LspServer {
    type Notification;
    type Request;
    type Response;
    type InitializeParams;
    type InitializeResult;
    type Error;

    async fn handle_notification(&mut self, msg: Self::Notification) -> Result<(), Self::Error>;
    async fn handle_request(&mut self, msg: Self::Request) -> Result<Self::Response, Self::Error>;
    async fn initialize(
        &mut self,
        params: Self::InitializeParams
    ) -> Result<Self::InitializeResult, Self::Error>;
}

trait Transport {
    fn wait_initial_message(&mut self) -> anyhow::Result<lsp_types::InitializeParams>;
    fn respond_initial_message(
        &mut self,
        result: lsp_types::InitializeResult
    ) -> anyhow::Result<()>;
}

pub async fn run_server() -> anyhow::Result<()> {
    use self::{server::BrowserServer, transport::Stdio};
    let mut transport = Stdio::new();
    let mut server = BrowserServer::new();
    let req = transport.wait_initial_message()?;
    let result = server.initialize(req).await?;
    transport.respond_initial_message(result)?;
    server.run(transport).await?;
    log::info!("server did shut down");
    Ok(())
}
