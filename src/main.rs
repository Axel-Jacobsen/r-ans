use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to calculate the entropy of
    filepath: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let f = File::open(&args.filepath)?;
    let mut reader = BufReader::new(f);

    let mut buf = [0u8; 4 * 1024]; // 4 kb buffer, probably too small
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

    let entropy = -byte_freq
        .iter()
        .filter(|&&x| x != 0)
        .map(|&x| {
            let p = x as f64 / tot_count as f64;
            p * f64::log(p, 2.0)
        })
        .sum::<f64>();

    println!("H = {entropy}");

    Ok(())
}
