mod data;
mod utils;
use crate::data::*;
use crate::utils::command_line::*;

fn main() {
    let blah = CommandLineOptionsProvider;
    let result = blah.get_command_line_options();

    if let Some(options) = result {
        println!("{}", options.configuration);
        println!("{}", options.source);
        println!("{}", options.destination);
    }

    println!("hello world");
}
