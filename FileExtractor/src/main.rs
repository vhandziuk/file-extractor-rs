use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path;
use std::path::Path;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use walkdir::WalkDir;

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

struct FileInfoData {
    name: String,
    directory_name: String,
}

fn main() {
    // configure logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // parse command line arguments
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

            // check .ZIP files in the source directory
            let archives = WalkDir::new(source_path)
                .into_iter()
                .filter_map(|e| match e {
                    Ok(fs_entry) => {
                        if let Some(file_name) = fs_entry.file_name().to_str() {
                            if file_name.to_lowercase().ends_with(".zip") {
                                match fs_entry.into_path().into_os_string().into_string() {
                                    Ok(file_path) => Some(file_path),
                                    Err(_) => None,
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .collect::<Vec<_>>();

            // reading .CSV configuration file
            if let Ok(lines) = read_lines(configuration_path.as_str()) {
                let file_infos = lines
                    .filter_map(|line| match line {
                        Ok(l) => {
                            let splitted = l.split(',').collect::<Vec<_>>();
                            Some(FileInfoData {
                                name: String::from(splitted[0]),
                                directory_name: String::from(splitted[1]),
                            })
                        }
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>();

                file_infos
                    .into_iter()
                    .for_each(|b| info!("{} {}", b.name, b.directory_name));
            }
        }
        Err(error) => {
            info!("could not parse command line options {}", error)
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
