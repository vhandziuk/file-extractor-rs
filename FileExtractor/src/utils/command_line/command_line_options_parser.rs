use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CommandLineOptionsParser {
    #[clap(short, long, help = "path to a CSV configuration file")]
    pub configuration: Option<String>,

    #[clap(short, long, help = "path to the source directory")]
    pub source: Option<String>,

    #[clap(short, long, help = "path to the destination directory")]
    pub destination: Option<String>,
}
