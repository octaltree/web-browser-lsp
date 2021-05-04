mod lsp;

use super::{LspServer, Transport};

#[derive(Debug, Default)]
pub(super) struct Worker<T: Transport> {
    transport: T,
    shutdown_requested: bool
}

impl<T: Transport> Worker<T>
where
    T: Transport<
            InitializeParams = <Self as LspServer>::InitializeParams,
            InitializeResult = <Self as LspServer>::InitializeResult
        > + Send
{
    pub(super) fn new(transport: T) -> Self {
        Self {
            transport,
            shutdown_requested: false
        }
    }

    pub(super) async fn initialize(&mut self) -> anyhow::Result<()> {
        let req = self.transport.wait_initial_message()?;
        let result = <Self as LspServer>::initialize(self, req).await?;
        self.transport.respond_initial_message(result)?;
        Ok(())
    }

    pub(super) fn close(self) -> anyhow::Result<()> { self.transport.close() }

    pub(super) async fn run(&mut self) -> anyhow::Result<()>
    where
        T: Transport<Message = lsp_server::Message>
    {
        loop {
            let msg = self.transport.next_message()?;
            if self.handle_message(msg).await? {
                break;
            }
        }
        Ok(())
    }

    async fn handle_message(&mut self, msg: lsp_server::Message) -> anyhow::Result<bool>
    where
        T: Transport<Message = lsp_server::Message>
    {
        log::debug!("RECV: {:?}", msg);
        let should_close = match msg {
            lsp_server::Message::Request(req) => {
                let response = self.handle_request(req).await?;
                self.transport
                    .send(lsp_server::Message::Response(response))?;
                false
            }
            lsp_server::Message::Notification(notif) => self.handle_notification(notif).await?,
            lsp_server::Message::Response(_) => todo!()
        };
        Ok(should_close)
    }
}
