use clap::Parser;
use std::env;
use std::path;
use tracing::{error, info};

use super::CommandLineOptions;
use super::CommandLineOptionsParser;

pub trait ICommandLineOptionsProvider {
    fn get_command_line_options(&self) -> Option<CommandLineOptions>;
}

pub struct CommandLineOptionsProvider;

impl ICommandLineOptionsProvider for CommandLineOptionsProvider {
    fn get_command_line_options(&self) -> Option<CommandLineOptions> {
        match CommandLineOptionsParser::try_parse() {
            Ok(options) => {
                info!("successfully parsed command line arguments");

                let source_path = match options.source {
                    Some(source) => source,
                    None => get_current_dir(),
                };
                let destination_path = match options.destination {
                    Some(destination) => destination,
                    None => String::from(source_path.as_str()),
                };
                let configuration_path = match options.configuration {
                    Some(configuration) => configuration,
                    None => add_configuration_filename(source_path.as_str()),
                };

                Some(CommandLineOptions {
                    configuration: configuration_path,
                    source: source_path,
                    destination: destination_path,
                })
            }
            Err(error) => {
                error!("failed to parse command line arguments. {}", error);
                None
            }
        }
    }
}

fn get_current_dir() -> String {
    return env::current_dir()
        .expect("retrieving current directory failed")
        .into_os_string()
        .into_string()
        .unwrap_or_default();
}

fn add_configuration_filename(path: &str) -> String {
    let mut configuration = path::PathBuf::from(path);
    configuration.extend(&["configuration.csv"]);
    configuration
        .into_os_string()
        .into_string()
        .unwrap_or_default()
}
