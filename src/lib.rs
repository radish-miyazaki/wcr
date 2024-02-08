use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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
    name: String,
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn new(name: String, num_lines: usize, num_words: usize, num_bytes: usize, num_chars: usize) -> Self {
        FileInfo {
            name,
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }
    }

    fn stdout_by_args(&self, args: &Args) {
        println!(
            "{}{}{}{}{}",
            format_field(args.lines, self.num_lines),
            format_field(args.words, self.num_words),
            format_field(args.bytes, self.num_bytes),
            format_field(args.chars, self.num_chars),
            if self.name == "-" { "".to_string() } else { format!(" {}", self.name) }
        );
    }
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

pub fn count(mut file: impl BufRead, name: String) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        let line_bytes = file.read_line(&mut buf)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += buf.split_whitespace().count();
        num_chars += buf.chars().count();

        buf.clear();
    }

    Ok(FileInfo::new(name, num_lines, num_words, num_bytes, num_chars))
}

fn format_field(show: bool, value: usize) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        String::new()
    }
}

pub fn run(args: Args) -> MyResult<()> {
    let mut total_num_lines = 0;
    let mut total_num_words = 0;
    let mut total_num_bytes = 0;
    let mut total_num_chars = 0;

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => {
                let file = open(filename)?;
                let file_info = count(file, filename.to_string())?;
                file_info.stdout_by_args(&args);

                total_num_lines += file_info.num_lines;
                total_num_words += file_info.num_words;
                total_num_bytes += file_info.num_bytes;
                total_num_chars += file_info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        let total_info = FileInfo::new(
            "total".to_string(),
            total_num_lines,
            total_num_words,
            total_num_bytes,
            total_num_chars,
        );
        total_info.stdout_by_args(&args);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{count, FileInfo, format_field};

    #[test]
    fn test_count() {
        let name = "test".to_string();
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text), name);

        assert!(info.is_ok());
        let expected = FileInfo::new("test".to_string(), 1, 10, 48, 48);
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(false, 1), "");
        assert_eq!(format_field(true, 3), "       3");
        assert_eq!(format_field(true, 10), "      10");
    }
}
