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

pub fn run(args: Args) -> MyResult<()> {
    println!("{:#?}", args);
    Ok(())
}
