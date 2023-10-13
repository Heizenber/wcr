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
    chars: bool
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename)
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