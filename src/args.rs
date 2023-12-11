use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub filename: PathBuf,

    /// Baud rate for serial. Default is 9600
    #[arg(short, long)]
    pub baud_rate: Option<u32>,
}
