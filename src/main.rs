use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::io;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args: Cli = Cli::from_args();
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    for line in reader.by_ref().lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
