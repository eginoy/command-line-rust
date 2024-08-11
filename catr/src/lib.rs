use clap::{value_parser, Arg, ArgAction, Command};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf) => {
                let mut r = BufReader::new(buf);
                let mut text = String::from("");
                _ = r.read_to_string(&mut text);

                if config.number_lines {
                    print!("{}", format_number_lines(&text))
                } else if config.number_nonblank {
                    print!("{}", format_number_nonblank(&text))
                } else {
                    print!("{}", &text)
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn format_number_lines(text: &str) -> String {
    let mut result = String::from("");
    let mut i = 1;
    for line in text.to_string().lines() {
        result += format!("     {}\t{}\n", i, line).as_str();
        i += 1;
    }

    result
}

pub fn format_number_nonblank(text: &str) -> String {
    let mut result = String::from("");
    let mut i = 1;
    for line in text.to_string().lines() {
        if line.is_empty() {
            result += "\n";
        } else {
            result += format!("     {}\t{}\n", i, line).as_str();
            i += 1;
        };
    }

    result
}

pub fn get_args() -> MyResult<Config> {
    let m = Command::new("catr")
        .version("0.1.0")
        .author("eginoy")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .default_value("-")
                .help("concatenate files"),
        )
        .arg(
            Arg::new("number_lines")
                .value_name("NUMBER_LINE")
                .long("number")
                .short('n')
                .help("number all output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .value_name("NUMBER_NONBLANK")
                .long("number-nonblank")
                .short('b')
                .help("number nonempty output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    let files: Vec<String> = m.get_many("files").unwrap().cloned().collect();

    Ok(Config {
        files: files,
        number_lines: m.get_flag("number_lines"),
        number_nonblank: m.get_flag("number_nonblank"),
    })
}
