use tokio;
use clap;
use env_logger;
use log::{self, LevelFilter};
use std::env;
use std::process;
use web_monitor::{run, Args};


#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut exit_code = 0;

    match Args::try_parse(env::args_os()) {
        Ok(args) => {
            let res = run(args).await;
            if let Err(e) = res {
                log::error!("{}", e);
                exit_code = 1;
            }
        }
        Err(e) => match e.kind() {
            clap::ErrorKind::DisplayHelp | clap::ErrorKind::DisplayVersion => {
                println!("{}", e);
            }
            _ => {
                log::error!("{}", e);
                exit_code = 1;
            }
        }
    }

    process::exit(exit_code);
}
