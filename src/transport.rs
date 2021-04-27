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
    fn wait_initial_message(&mut self) -> anyhow::Result<lsp_types::InitializeParams> {
        let (initialize_id, initialize_params) = self.conn.initialize_start()?;
        log::info!("InitializeParams: {}", initialize_params);
        self.initialize_id = Some(initialize_id);
        let initialize_params: lsp_types::InitializeParams =
            serde_json::from_value(initialize_params)?;
        Ok(initialize_params)
    }

    fn respond_initial_message(
        &mut self,
        result: lsp_types::InitializeResult
    ) -> anyhow::Result<()> {
        let initialize_id = self
            .initialize_id
            .take()
            .ok_or_else(|| anyhow::anyhow!("initialize_id is None"))?;
        let initialize_result = serde_json::to_value(result)?;
        self.conn
            .initialize_finish(initialize_id, initialize_result)?;
        Ok(())
    }

    fn close(self) -> anyhow::Result<()> { Ok(self.io_threads.join()?) }
}
