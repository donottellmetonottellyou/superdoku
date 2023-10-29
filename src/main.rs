mod cli;
mod gui;

use clap::Parser;

/// Superdoku, a Sudoku solver
#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    /// Use the command line interface version of Superdoku
    #[arg(short, long, default_value_t = true)]
    pub cli: bool,
}

fn main() {
    let args = Args::parse();

    if args.cli {
        cli::main();
    } else {
        gui::main();
    }
}
