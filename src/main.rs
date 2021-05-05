use std::{path::Path, process};
use structopt::StructOpt;
use web_browser_lsp::{run_server, TempDir};

#[tokio::main]
async fn main() {
    let temp_dir = TempDir::new();
    init_log(temp_dir.as_path()).unwrap();
    let opt: Opt = Opt::from_args();
    if let Err(err) = run(opt, temp_dir).await {
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

async fn run(opt: Opt, temp_dir: TempDir) -> anyhow::Result<()> {
    match opt.sub_command.as_ref() {
        Some(SubCommand::Server(_)) => run_server(temp_dir).await?,
        _ => run_server(temp_dir).await?
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
