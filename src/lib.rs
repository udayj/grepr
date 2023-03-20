use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {

    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("grepr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust grep")
                    .arg(
                        Arg::with_name("pattern")
                            .value_name("PATTERN")
                            .help("Search pattern")
                            .required(true)
                    )
                    .arg(
                        Arg::with_name("files")
                            .value_name("FILES")
                            .help("FILE/DIRECTORIES to Search")
                            .multiple(true)
                            .default_value("-")
                    )
                    .arg(
                        Arg::with_name("recursive")
                            .short("r")
                            .long("recursive")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("count")
                            .short("c")
                            .long("count")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("invert_match")
                            .short("v")
                            .long("invert_match")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("insensitive")
                            .short("i")
                            .long("insensitive")
                            .takes_value(false)
                    )
                    .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern)
                                                    .case_insensitive(matches.is_present("insensitive"))
                                                    .build()
                                                    .map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;
    let files = matches.values_of_lossy("files").unwrap();
    
    Ok(
        Config {
            pattern,
            files,
            recursive: matches.is_present("recursive"),
            count: matches.is_present("count"),
            invert_match: matches.is_present("invert_match")
        }
    )
}

pub fn run(config: Config) -> MyResult<()> {

    println!("{:#?}", config);
    Ok(())
}