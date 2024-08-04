mod file_statistics;

use crate::file_statistics::SymbolStatistics;

use std::io;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn entropy_of_file(f: PathBuf) -> io::Result<f64> {
    Ok(-file_statistics::SymbolStatistics::new(f)?
        .normalize()
        .symbol_probabilities
        .iter()
        .filter(|&&p| p != 0.)
        .map(|&p| p * f64::log(p, 2.0))
        .sum::<f64>())
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// What to do
    #[command(subcommand)]
    command: Commands,
}

/// Enum for subcommands
#[derive(Subcommand)]
enum Commands {
    /// Compress a file
    Compress {
        #[arg(value_name = "FILE")]
        filepath: PathBuf,
    },
    /// Calculate the entropy of a file
    Entropy {
        #[arg(value_name = "FILE")]
        filepath: PathBuf,
    },
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let output = match args.command {
        Commands::Entropy { filepath } => entropy_of_file(filepath)?,
        Commands::Compress { filepath } => compress(filepath)?,
    };

    println!("{output}");

    Ok(())
}
