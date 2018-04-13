/// Filter a log file by matching multiple regular expressions
/// Reads a file named `application.log` and only outputs the lines
/// containing "version X.X.X", some IP address followed by port 443,
/// or a specific warning.
extern crate regex;
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::RegexSetBuilder;

error_chain!{
    foreign_links {
        IOError(std::io::Error);
        RegexError(regex::Error);
    }
}

fn run() -> Result<()> {
    let file = File::open("application.log")?;
    let bufreader = BufReader::new(file);
    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ]).case_insensitive(true).build()?;

    bufreader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));
    Ok(())
}

quick_main!(run);
