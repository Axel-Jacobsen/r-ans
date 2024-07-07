use std::io;
use std::io::Read;
use std::fs::File;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to calculate the entropy of
    filepath: String,
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut f = File::open(args.filepath.to_string())?;
    let mut buf = [0u8; 4 * 1024];  // 4 kb buffer, probably too small
    let mut byte_freq = [0u64; 256];
    let mut tot_count = 0u64;

    loop {
        let bytes_read = f.read(&mut buf)?;
        if bytes_read == 0 { break };
        tot_count += bytes_read as u64;

        for i in 0..bytes_read {
            byte_freq[buf[i] as usize] += 1;
        }
    }

    let entropy = - byte_freq.into_iter().map(|x| {
        if x != 0 {
            let p = x as f64 / tot_count as f64;
            p * f64::log(p, 2.0)
        } else {
            0f64
        }
    }).sum::<f64>();

    println!("H = {entropy}");

    Ok(())
}
