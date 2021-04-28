use super::Transport;
use lsp_server::{Connection, IoThreads, RequestId};

pub(super) struct Stdio {
    conn: Connection,
    io_threads: IoThreads,
    initialize_id: Option<RequestId>
}

impl Stdio {
    pub(super) fn new() -> Self {
        let (conn, io_threads) = Connection::stdio();
        Self {
            conn,
            io_threads,
            initialize_id: None
        }
    }
}

impl Transport for Stdio {
    type Message = lsp_server::Message;
    type InitializeParams = lsp_types::InitializeParams;
    type InitializeResult = lsp_types::InitializeResult;

    fn wait_initial_message(&mut self) -> Result<lsp_types::InitializeParams, anyhow::Error> {
        let (initialize_id, initialize_params) = self.conn.initialize_start()?;
        log::info!("InitializeParams: {}", initialize_params);
        self.initialize_id = Some(initialize_id);
        let initialize_params: lsp_types::InitializeParams =
            serde_json::from_value(initialize_params)?;
        Ok(initialize_params)
    }

    fn respond_initial_message(
        &mut self,
        result: Self::InitializeResult
    ) -> Result<(), anyhow::Error> {
        let initialize_id = self
            .initialize_id
            .take()
            .ok_or_else(|| anyhow::anyhow!("initialize_id is None"))?;
        let initialize_result = serde_json::to_value(result)?;
        self.conn
            .initialize_finish(initialize_id, initialize_result)?;
        Ok(())
    }

    fn send(&mut self, msg: Self::Message) -> Result<(), anyhow::Error> {
        Ok(self.conn.sender.send(msg)?)
    }

    fn next_message(&mut self) -> Result<Self::Message, anyhow::Error> {
        Ok(self.conn.receiver.recv()?)
    }

    fn close(self) -> Result<(), anyhow::Error> { Ok(self.io_threads.join()?) }
}
