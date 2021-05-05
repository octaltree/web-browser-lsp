#[macro_use]
extern crate async_trait;

mod transport;
mod worker;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process
};

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

pub async fn run_server(temp_dir: TempDir) -> anyhow::Result<()> {
    use self::{transport::Stdio, worker::Worker};
    let transport = Stdio::new();
    let mut worker = Worker::new(transport);
    worker.initialize().await?;
    worker.run().await?;
    // client may kill on stdio closed
    temp_dir.remove();
    log::info!("exit success");
    worker.close()?;
    Ok(())
}

pub struct TempDir {
    path: PathBuf
}

impl TempDir {
    /// # Panics
    /// Panics if io::Error is unwrapped
    pub fn new() -> Self {
        let id = process::id();
        let base = env::temp_dir();
        let path = base.join(format!("web-browser-lsp-{}", id));
        fs::create_dir(&path).unwrap();
        Self { path }
    }

    pub fn as_path(&self) -> &Path { &self.path }

    fn remove(self) { fs::remove_dir_all(self.as_path()).ok(); }
}
