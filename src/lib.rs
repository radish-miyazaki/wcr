use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(name = "wc")]
#[command(version = "0.1.0")]
#[command(about = "Rust wc")]
#[command(author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>")]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, help = "Show line count")]
    lines: bool,
    #[arg(short, long, help = "Show word count")]
    words: bool,
    #[arg(short = 'c', long = "bytes", help = "Show byte count", conflicts_with = "chars")]
    bytes: bool,
    #[arg(short = 'm', long = "chars", help = "Show character count")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Args> {
    let mut args = Args::parse();
    let lines = args.lines;
    let words = args.words;
    let bytes = args.bytes;
    let chars = args.chars;

    if [lines, words, bytes, chars].iter().all(|&x| !x) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => {}
        }
    }

    Ok(())
}
