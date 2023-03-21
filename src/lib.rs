use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use walkdir::{WalkDir, DirEntry};

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

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {

    let mut result = vec![];

    
    for path in paths {

        for actual_path in WalkDir::new(path) {

            match actual_path {

                Err(e) => {result.push(Err(From::from(format!("{}", e))));}
                Ok(val) => {
                    
                    if !recursive && val.file_type().is_dir() {
                        result.push(Err(String::from(format!("{} is a directory", val.path().to_str().unwrap()))));
                        break;
                    }
                    if !val.file_type().is_dir()  {
                        result.push(Ok(String::from(val.path().to_str().unwrap())));
                    }
                }

            }
        }
        
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{find_files};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files =
            find_files(&["./tests/inputs/fox.txt".to_string()], false);
        
        println!("1");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        println!("2");
        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);

        println!("3");
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        println!("4");
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}