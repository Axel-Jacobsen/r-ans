mod file_statistics;

use crate::file_statistics::SymbolStatistics;

use std::io;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn entropy_of_file(f: PathBuf) -> io::Result<f64> {
    let ps = file_statistics::SymbolStatistics::new(f)?
        .normalize()
        .symbol_probabilities;

    let entropy = -ps
        .iter()
        .filter(|&&p| p != 0.)
        .map(|&p| p * f64::log(p, 2.0))
        .sum::<f64>();

    Ok(entropy)
}

fn compress(f: PathBuf) -> io::Result<f64> {
    let statistics = SymbolStatistics::new(f)?;

    // construct the cdf array
    let mut cdf_arr = [0u64; 256];
    for (i, count) in statistics.symbol_counts.iter().enumerate() {
        cdf_arr[i] = count + if i == 0 { 0 } else { cdf_arr[i - 1] };
    }

    let _cdf = |s: u8| -> u64 {
        if s == 0 {
            0
        } else {
            cdf_arr[(s - 1) as usize]
        }
    };

    Ok(0f64)
}

// TODO I don't like the ergonomics of this CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What to do
    #[command(subcommand)]
    command: Commands,

    /// Path to input file
    filepath: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Calculate entropy of the input file
    Entropy,

    /// Compress the input file
    Compress,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let output = match args.command {
        Commands::Entropy => entropy_of_file(args.filepath)?,
        Commands::Compress => compress(args.filepath)?,
    };

    println!("{output}");

    Ok(())
}
