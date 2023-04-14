use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(
    author = "Arun M",
    version = "0.1.0",
    about = "Cat in Rust",
    long_about = "Looong cat in Rust"
)]
struct Cli {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long("number"),
        conflicts_with("number_nonblank_lines"),
        help = "Number lines"
    )]
    number_lines: bool,
    #[arg(
        short = 'b',
        long("number-nonblank"),
        conflicts_with("number_lines"),
        help = "Number non-blank lines"
    )]
    number_nonblank_lines: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("ERROR while opening file {filename}: {e}"),
            Ok(f) => {
                let mut blank_ln = 0;
                for (ln, line) in f.lines().enumerate() {
                    let line = line?;
                    if args.number_lines {
                        println!("{lno:6}\t {line}", lno = ln + 1)
                    } else if args.number_nonblank_lines {
                        if !line.is_empty() {
                            blank_ln += 1;
                            println!("{blank_ln:6}\t{line}");
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
