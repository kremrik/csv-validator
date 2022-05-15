use clap::Parser;

/// A CSV validation command-line tool
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to CSV file
    #[clap(long)]
    pub csv_file: String,

    /// Path to constraints JSON file
    #[clap(long)]
    pub constraints_file: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
