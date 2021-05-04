use std::{
    env, fs,
    path::{Path, PathBuf},
    process
};
use structopt::StructOpt;
use web_browser_lsp::run_server;

#[tokio::main]
async fn main() {
    let temp_dir = TempDir::new();
    init_log(temp_dir.path()).unwrap();
    let opt: Opt = Opt::from_args();
    if let Err(err) = run(opt).await {
        log::error!("{:?}", err);
        process::exit(101);
    }
}

fn init_log(temp_dir: &Path) -> anyhow::Result<()> {
    use log::LevelFilter;
    use log4rs::{
        append::file::FileAppender,
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder
    };
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(temp_dir.join("debug.log"))?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace)
        )?;
    log4rs::init_config(config)?;
    Ok(())
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    match opt.sub_command.as_ref() {
        Some(SubCommand::Server(_)) => run_server().await?,
        _ => run_server().await?
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    pub sub_command: Option<SubCommand>
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Server(Server)
}

#[derive(Debug, StructOpt)]
struct Server {}

struct TempDir {
    path: PathBuf
}

impl TempDir {
    /// # Panics
    /// Panics if io::Error is unwrapped
    fn new() -> Self {
        let id = process::id();
        let base = env::temp_dir();
        let path = base.join(format!("web-browser-lsp-{}", id));
        fs::create_dir(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path { &self.path }
}

impl Drop for TempDir {
    fn drop(&mut self) { fs::remove_dir_all(&self.path()).ok(); }
}
