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

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
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

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn stdout_file_info(args: &Args, file_info: &FileInfo, filename: &str) {
    let mut output = String::new();

    if args.lines {
        output.push_str(&format!("{:8}", file_info.num_lines));
    }

    if args.words {
        output.push_str(&format!("{:8}", file_info.num_words));
    }

    if args.bytes {
        output.push_str(&format!("{:8}", file_info.num_bytes));
    } else if args.chars {
        output.push_str(&format!("{:8}", file_info.num_chars));
    }

    if filename != "-" {
        output.push_str(&format!(" {}", filename));
    }
    println!("{}", output);
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
                let file_info = open(filename).and_then(count)?;
                stdout_file_info(&args, &file_info, filename);

                total_num_lines += file_info.num_lines;
                total_num_words += file_info.num_words;
                total_num_bytes += file_info.num_bytes;
                total_num_chars += file_info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        let total = FileInfo {
            num_lines: total_num_lines,
            num_words: total_num_words,
            num_bytes: total_num_bytes,
            num_chars: total_num_chars,
        };

        stdout_file_info(&args, &total, "total");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
