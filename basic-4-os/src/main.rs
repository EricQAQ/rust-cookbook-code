/// 1. Run an external command and process stdout
/// runs `git log --oneline` as an external Command and inspects its output using
/// `Regex` to get the hash and message of the last 5 commits.
/// Regex: a crate for regular expressions in Rust.
#[macro_use]
extern crate error_chain;
extern crate regex;

use std::process::Command;
use regex::Regex;

error_chain! {
    foreign_links {
        IOError(std::io::Error);
        RegexError(regex::Error);
        OutputError(std::string::FromUtf8Error);
    }
}

#[derive(Debug, Clone, Default)]
struct CommitObj {
    hash: String,
    message: String,
}

fn run_1() -> Result<()> {
    // Command的output方法, 返回Output结构体, 有三个元素:
    // 1. status: The status (exit code) of the process.
    // 2. stdout: The data that the process wrote to stdout.
    // 3. stderr: The data that the process wrote to stderr.
    let output = Command::new("git").arg("log").arg("--oneline").output()?;
    if !output.status.success() {
        // bail宏是error_chain提供的一个宏, 用来退出当前函数, 返回值为一个错误
        bail!("Command executed with failing error code.");
    }

    let pattern = Regex::new(r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message")?;
    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|context| CommitObj {
            hash: context[1].to_string(),
            message: context[2].trim().to_string(),
        })
        .take(5)
        .for_each(|item| println!("{:?}", item));

    Ok(())
}

/// 2. Run an external command passing it stdin and check for an error code
/// Opens a python interperter using an external Command and passes it a python
/// statement for execution. Output of said statement is then parsed.
use std::collections::HashSet;
use std::io::Write;
use std::process::Stdio;

fn run_2() -> Result<()> {
	// 触发python命令
    let mut child = Command::new("python")
		.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
		.spawn()?;
	// 将后续命令写入python的stdin
	child.stdin.as_mut()
		.ok_or("Child process stdin has not been captured!")?
		.write_all(b"import this; copyright(); credits(); exit()")?;
	// 同步等待命令执行完毕, 并且收集到所有stdout和stderr的结果, 返回Output
    let output = child.wait_with_output()?;
	// 处理输出结果
    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
		println!("{:#?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        bail!("External command failed: \n {}", err)
    }
}

/// 3. Run piped external commands
/// Shows up to the 10th biggest files and subdirectories in the current working
/// directory. It is equivalent to run: `du -ah . | sort -gr | head -n 10`
fn run_3() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let du_output = Command::new("du").arg("-ah").arg(&current_dir)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap();

    let sort_output = Command::new("sort").arg("-gr")
        .stdin(du_output)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap();

    let head_output = Command::new("head").arg("-n").arg("10")
        .stdin(sort_output)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    println!(
        "Top 10 biggest files and directories in '{}':\n{}",
        current_dir.display(),
        String::from_utf8(head_output.stdout)?
    );
    Ok(())
}

/// 4. Redirect both stdout and stderr of child process to the same file
/// `ls . oops >out.txt 2>&1`
use std::fs::File;

fn run_4() -> Result<()> {
    let outputs = File::create("nohup.out")?;
    // File::try_clone : reference the same file handle for stdout and stderr,
    //                   it will ensure that both handles write with the same cursor position
    let errors = outputs.try_clone()?;
    let current_dir = std::env::current_dir()?;
    Command::new("ls")
        .arg(current_dir)
        .arg("oops")
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

/// 5. Continuously process child process' outputs
/// `journalctl | grep usb`
use std::io::{BufReader, BufRead};

fn run_5() -> Result<()> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .unwrap();
    let reader = BufReader::new(stdout);
    reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));
    Ok(())
}

// quick_main!(run_1);
// quick_main!(run_2);
// quick_main!(run_3);
// quick_main!(run_4);
quick_main!(run_5);
