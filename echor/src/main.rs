use clap::{arg, command, Parser};

#[derive(clap::Parser, Debug)]
#[command(
    author = "Arun M",
    version = "0.1.0",
    about = "Echo in Rust",
    long_about = "Long Echo in Rust"
)]
struct Cli {
    #[arg(required(true))]
    text: Vec<String>,

    #[arg(short = 'n', long)]
    omit_newline: bool,
}

fn main() {
    /* let matches = Command::new("echor")
    .version("0.1.0")
    .author("Arun Manivannan")
    .about("Echo on Rust")
    .arg(
        Arg::new("text")
            .value_name("TEXT")
            .required(true)
            .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
            .short('n')
            .action(ArgAction::SetTrue)
            .long("omit_newline")
            .help("Do not print newline"),
    )
    .get_matches();

    println!("{matches:?}");*/
    let args = Cli::parse();
    let text = args.text.join(" ");
    let ending = if args.omit_newline { "" } else { "\n" };

    print!("{text}{ending}");
}

/*
Long Echo in Rust

Usage: echor [OPTIONS] --text <<TEXT>>

Options:
  -t, --text <<TEXT>>


  -n, --omit_newline <OMIT>
          [possible values: true, false]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version


 */

/*
Echo on Rust

Usage: echor [OPTIONS] --text <TEXT>

Options:
      --text <TEXT>   Input text
  -n, --omit_newline  Do not print newline
  -h, --help          Print help
  -V, --version       Print version

 */

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use assert_cmd::Command;
    use predicates::prelude::predicate;

    use super::*;

    type TestResult = Result<(), Box<dyn Error>>;

    fn run(args: &[&str], expected_file: &str) -> TestResult {
        let expected = fs::read_to_string(expected_file)?;
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.args(args).assert().success().stdout(expected);
        Ok(())
    }

    #[test]
    fn test_hello1() -> TestResult {
        let outfile = "tests/expected/hello1.txt";
        run(&["Hello there"], outfile)
    }

    #[test]
    fn test_hello2() -> TestResult {
        let outfile = "tests/expected/hello2.txt";
        run(&["Hello there"], outfile)
    }

    #[test]
    fn test_hello1n() -> TestResult {
        let outfile = "tests/expected/hello1.n.txt";
        run(&["Hello there", "-n"], outfile)
    }

    #[test]
    fn test_hello2n() -> TestResult {
        let outfile = "tests/expected/hello2.n.txt";
        run(&["Hello there", "-n"], outfile)
    }

    #[test]
    fn test_dies_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));

        Ok(())
    }
}
