mod lsp;

use super::{LspServer, Transport};

#[derive(Debug, Default)]
pub(super) struct Worker {
    shutdown_requested: bool
}

impl Worker {
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
        log::debug!("RECV: {:?}", msg);
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
