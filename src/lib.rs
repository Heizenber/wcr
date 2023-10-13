use clap::{Arg, App};
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self { num_lines: 0, num_words: 0, num_bytes: 0, num_chars: 0 } 
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        
        num_bytes += line_bytes;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo { num_lines, num_words, num_bytes, num_chars })
}

pub fn run(config: Config) -> MyResult<()> {
    let number_of_files = &config.files.len();
    let last_filename = &config.files[*number_of_files - 1];
    for filename in &config.files {
        let mut total = FileInfo::default();
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{:>8}{:>8}{:>8} {}",
                        info.num_lines,
                        info.num_words,
                        info.num_bytes,
                        filename
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Roman Popov <example@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .help("Show line count")
                .short("l")
                .long("lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .help("Show byte count")
                .short("c")
                .long("bytes")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .help("Show character count")
                .short("m")
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("words")
                .help("Show word count")
                .short("w")
                .long("words")
                .takes_value(false)
        )
        .get_matches();


    let files = matches.values_of_lossy("files").unwrap();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words, 
        bytes,
        chars
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::format_field;

    use super::{count, FileInfo};
    use std::io::Cursor;

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

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}

