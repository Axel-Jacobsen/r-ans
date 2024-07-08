use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

struct SymbolProbabilities {
    symbol_probabilities: [f64; 256],
}

struct SymbolStatistics {
    tot_count: u64,
    symbol_counts: [u64; 256],
}

impl SymbolStatistics {
    fn new(f: PathBuf) -> io::Result<SymbolStatistics> {
        let f = File::open(f)?;
        let mut reader = BufReader::new(f);

        let mut buf = [0u8; 1024 * 1024];
        let mut byte_freq = [0u64; 256];
        let mut tot_count = 0u64;

        loop {
            let bytes_read = reader.read(&mut buf)?;
            if bytes_read == 0 {
                break;
            };
            tot_count += bytes_read as u64;

            for &byte in buf.iter().take(bytes_read) {
                byte_freq[byte as usize] += 1;
            }
        }

        Ok(SymbolStatistics {
            tot_count,
            symbol_counts: byte_freq,
        })
    }

    fn normalize(&self) -> SymbolProbabilities {
        let normalized_frequencies: [f64; 256] = self
            .symbol_counts
            .into_iter()
            .map(|x| x as f64 / self.tot_count as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        SymbolProbabilities {
            symbol_probabilities: normalized_frequencies,
        }
    }
}

fn entropy_of_file(f: PathBuf) -> io::Result<f64> {
    let ps = SymbolStatistics::new(f)?.normalize().symbol_probabilities;

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
            cdf_arr[s as usize]
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
