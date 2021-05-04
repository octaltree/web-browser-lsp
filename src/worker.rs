mod lsp;

use super::{LspServer, Transport};
use std::time::Instant;

// pub(crate) type ResponseHandler = fn(&mut Worker, lsp_server::Response);
type ResponseHandler<T> = fn(&mut Worker<T>, lsp_server::Response) -> anyhow::Result<()>;
type TransportQueue<T> = lsp_server::ReqQueue<(String, Instant), ResponseHandler<T>>;

pub(super) struct Worker<T: Transport> {
    transport: T,
    transport_queue: TransportQueue<T>,
    shutdown_requested: bool
}

impl<T> Worker<T>
where
    T: Transport<
            InitializeParams = <Self as LspServer>::InitializeParams,
            InitializeResult = <Self as LspServer>::InitializeResult,
            Message = lsp_server::Message
        > + Send
{
    pub(super) fn new(transport: T) -> Self {
        Self {
            transport,
            transport_queue: TransportQueue::default(),
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
            // Should I make errors silent?
            if self.handle_message(msg).await? {
                break;
            }
        }
        log::info!("exit");
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
            lsp_server::Message::Response(resp) => {
                let handler = self.transport_queue.outgoing.complete(resp.id.clone());
                handler(self, resp)?;
                false
            }
        };
        Ok(should_close)
    }
}
