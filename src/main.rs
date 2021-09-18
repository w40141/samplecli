use clap::{Clap};

#[derive(Clap, Debug)]
#[clap(
    name = "My PRN program",
    version = "1.0.0",
    author = "Daisuke Oku",
    about = "Super awesome sample RPN caluculator"
)]

struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

pub fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}
