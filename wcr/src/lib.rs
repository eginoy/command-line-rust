use core::panic;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use clap::Parser;
use std::result::Result::Ok;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short('l'), long, default_value("false"))]
    lines: bool,

    #[arg(short('w'), long, default_value("false"))]
    words: bool,

    #[arg(short('c'), long, default_value("false"), conflicts_with("chars"))]
    bytes: bool,

    #[arg(short('m'), long)]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run(arg: Arg) -> Result<()> {
    let is_print_total = arg.files.len() > 1;
    let mut sum_lines = 0;
    let mut sum_words = 0;
    let mut sum_bytes = 0;
    let mut sum_chars = 0;

    for filename in &arg.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let info = count(file)?;
                let filename = filename;
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, arg.lines),
                    format_field(info.num_words, arg.words),
                    format_field(info.num_bytes, arg.bytes),
                    format_field(info.num_chars, arg.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", filename)
                    }
                );

                if is_print_total {
                    sum_lines += info.num_lines;
                    sum_words += info.num_words;
                    sum_bytes += info.num_bytes;
                    sum_chars += info.num_chars;
                }
            }
        }
    }
    if is_print_total {
        println!(
            "{}{}{}{} total",
            format_field(sum_lines, arg.lines),
            format_field(sum_words, arg.words),
            format_field(sum_bytes, arg.bytes),
            format_field(sum_chars, arg.chars)
        );
    }
    Ok(())
}

pub fn arg() -> Arg {
    /*
        フラグ指定なしのデフォルト値の場合はArgPredicate::IsPresentで無視されるので、Some("false")の条件に該当せず
        chars以外のデフォルト値がTrueになる想定だが、ArgPredicate::IsPresentのバグでデフォルト値まで判定対象となってしまう。
        version 5.xで修正される予定の模様
        https://github.com/clap-rs/clap/issues/4918
    */
    let mut arg = Arg::parse();
    if [&arg.lines, &arg.words, &arg.bytes, &arg.chars]
        .iter()
        .all(|v| v == &&false)
    {
        arg.lines = true;
        arg.words = true;
        arg.bytes = true;
    }

    arg
}

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line_buf = String::new();

    loop {
        let line_bytes = file.read_line(&mut line_buf)?;
        if line_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_words += &line_buf.split_whitespace().count();
        num_bytes += line_bytes;
        num_chars += &line_buf.chars().count();
        line_buf.clear();
    }

    Ok(FileInfo {
        num_lines: num_lines,
        num_words: num_words,
        num_bytes: num_bytes,
        num_chars: num_chars,
    })
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
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), format!("{:>8}", 3));
        assert_eq!(format_field(10, true), format!("{:>8}", 10));
    }
}
