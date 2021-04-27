use std::process;
use structopt::StructOpt;
use web_browser_lsp::run_server;

#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    if let Err(err) = run(opt).await {
        eprintln!("{:?}", err);
        process::exit(101);
    }
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
