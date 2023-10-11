use clap::{Arg, App};
use std::error::Error;


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
    println!("{:#?}", config);
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
                .value_name("LINES")
                .help("print the newline counts")
                .short("l")
                .long("lines")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("print the byte counts")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .conflicts_with("chars")
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("print the character counts")
                .short("m")
                .long("chars")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("print the word counts")
                .short("w")
                .long("words")
                .takes_value(true)
        )
        .get_matches();


    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches.is_present("lines");
    let words = matches.is_present("words");
    let bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    Ok(Config {
        files,
        lines,
        words, 
        bytes,
        chars
    })
}