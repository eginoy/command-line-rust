use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short('n'),
        long("lines"),
        conflicts_with("bytes"),
        default_value("10"),
        value_parser=clap::value_parser!(usize)
    )]
    lines: usize,

    #[arg(short('c'), long("bytes"),value_parser=clap::value_parser!(usize))]
    bytes: Option<usize>,
}

pub fn run(arg: Arg) -> Result<()> {
    for (file_num, filename) in arg.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                let is_print_header = arg.files.len() > 1;
                if is_print_header == true {
                    print!(
                        "{}==> {} <==\n",
                        if file_num == 0 { "" } else { "\n" },
                        filename
                    );
                }

                if let Some(num_bytes) = arg.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;

                    let output = String::from_utf8_lossy(&buffer[..bytes_read]);
                    print!("{}", output);
                } else {
                    //lines print
                    let mut line = String::new();
                    for _ in 0..arg.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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

fn parse_positive_int(val: &str) -> Result<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(anyhow!("{}", val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
