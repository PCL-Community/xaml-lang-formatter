mod cli;
mod formatter;
mod grouping;
mod model;
mod parser;
mod writer;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(err) = formatter::run(cli) {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
