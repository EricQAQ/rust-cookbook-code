/// Read lines of strings from a file
///
/// Writes a three-line message to a file, then reads it back a line at a
/// time with the Lines iterator created by BufRead::lines.
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

error_chain! {
    foreign_links {
        FileFailure(std::io::Error);
    }
}

fn run() -> Result<()> {
    let path = "lines.txt";
    let mut output = File::create(path)?;   // 创建文件
    write!(output, "Rust\n💖\nFun")?;       // 写入三行文件

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

quick_main!(run);
