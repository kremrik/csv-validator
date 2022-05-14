use clap::Parser;

/// A CSV validation command-line tool
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to constraints JSON file
    #[clap(short, long)]
    pub constraints: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
