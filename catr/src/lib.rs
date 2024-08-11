use anyhow::Result;
use clap::Parser;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with("number_nonblank"))]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank: bool,
}

pub fn run(config: Arg) -> Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_number_nonblank = 1;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?; //シャドーイング
                    if config.number_lines {
                        println!("{:6}\t{line}", line_num + 1);
                    } else if config.number_nonblank {
                        if line.is_empty() {
                            println!()
                        } else {
                            println!("{line_number_nonblank:6}\t{line}",);
                            line_number_nonblank += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
