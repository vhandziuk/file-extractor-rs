use clap::Parser;
use std::env;
use std::path;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLineOptions {
    #[clap(short, long, help = "path to a CSV configuration file")]
    configuration: Option<String>,

    #[clap(short, long, help = "path to the source directory")]
    source: Option<String>,

    #[clap(short, long, help = "path to the destination directory")]
    destination: Option<String>,
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match CommandLineOptions::try_parse() {
        Ok(options) => {
            let source_path = match options.source {
                Some(source) => source,
                None => env::current_dir()
                    .expect("retrieving current directory failed")
                    .into_os_string()
                    .into_string()
                    .unwrap_or_default(),
            };
            let destination_path = match options.destination {
                Some(destination) => destination,
                None => String::from(source_path.as_str()),
            };
            let configuration_path = match options.configuration {
                Some(configuration) => configuration,
                None => {
                    let mut configuration = path::PathBuf::from(source_path.as_str());
                    configuration.extend(&["configuration.csv"]);
                    configuration
                        .into_os_string()
                        .into_string()
                        .unwrap_or_default()
                }
            };

            info!("source: {}", source_path);
            info!("destination: {}", destination_path);
            info!("configuration: {}", configuration_path);
        }
        Err(error) => {
            info!("could not parse command line options {}", error)
        }
    };
}
