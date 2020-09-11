use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};
use std::io;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();
    let file = File::open(&args.path)
        .with_context(|_| format!("Could not read file `{}`", &args.path.to_str().unwrap()))?;
    let mut reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in reader.by_ref().lines() {
        let line = line
            .with_context(|_| format!("Could not read line..."))?;
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", &line)
                .with_context(|_| format!("Could not write line to terminal"))?;
        }
    }
    Ok(())
}
