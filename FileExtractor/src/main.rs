use clap::Parser;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLineOptions {
    #[clap(short, long, help = "Path to a CSV configuration file")]
    configuration: Option<String>,

    #[clap(short, long, help = "Path to the source directory")]
    source: Option<String>,

    #[clap(short, long, help = "Path to the destination directory")]
    destination: Option<String>,
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match CommandLineOptions::try_parse() {
        Ok(options) => {
            let configuration = match options.configuration {
                Some(config) => config,
                None => {
                    info!("configuration option not provided, using current directory as default");
                    match env::current_dir() {
                        Ok(curr_path) => curr_path.as_os_str(),
                        Err => String::from(""),
                    }
                }
            };
        }
        Err(error) => {
            info!("could not parse command line options {}", error)
        }
    };
}
