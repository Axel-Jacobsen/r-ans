use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

pub struct SymbolProbabilities {
    pub symbol_probabilities: [f64; 256],
}

pub struct SymbolStatistics {
    pub tot_count: u64,
    pub symbol_counts: [u64; 256],
}

impl SymbolStatistics {
    pub fn new(f: PathBuf) -> io::Result<SymbolStatistics> {
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

    pub fn normalize(&self) -> SymbolProbabilities {
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
