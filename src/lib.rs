#[macro_use]
extern crate async_trait;

mod transport;
mod worker;

#[async_trait]
trait LspServer {
    type Notification;
    type Request;
    type Response;
    type InitializeParams;
    type InitializeResult;

    async fn initialize(
        &mut self,
        params: Self::InitializeParams
    ) -> Result<Self::InitializeResult, anyhow::Error>;

    async fn handle_notification(&mut self, msg: Self::Notification)
        -> Result<bool, anyhow::Error>;
    async fn handle_request(&mut self, msg: Self::Request)
        -> Result<Self::Response, anyhow::Error>;
}

trait Transport {
    type Message;
    type InitializeParams;
    type InitializeResult;

    fn wait_initial_message(&mut self) -> Result<Self::InitializeParams, anyhow::Error>;
    fn respond_initial_message(
        &mut self,
        result: Self::InitializeResult
    ) -> Result<(), anyhow::Error>;

    fn next_message(&mut self) -> Result<Self::Message, anyhow::Error>;
    fn send(&mut self, msg: Self::Message) -> Result<(), anyhow::Error>;

    fn close(self) -> Result<(), anyhow::Error>;
}

pub async fn run_server() -> anyhow::Result<()> {
    use self::{transport::Stdio, worker::Worker};
    let mut transport = Stdio::new();
    let mut server = Worker::new();
    let req = transport.wait_initial_message()?;
    let result = server.initialize(req).await?;
    transport.respond_initial_message(result)?;
    let result = server.run(&mut transport).await;
    transport.close()?;
    result?;
    log::info!("server did shut down");
    Ok(())
}
