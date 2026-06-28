use clap::Parser;

use xaml_lang_formatter::cli::Cli;
use xaml_lang_formatter::formatter::run;

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
