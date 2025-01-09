use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let mut command = Command::new("cargo")
        .arg("check")
        .env("RUSTC_WRAPPER", "wrapper-driver")
        .env("WRAPPER_LOG_LEVEL", "DEBUG")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Handle stdout
    if let Some(stdout) = command.stdout.take() {
        let stdout_reader = BufReader::new(stdout);
        std::thread::spawn(move || {
            stdout_reader.lines().for_each(|line| {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            });
        });
    }

    let status = command.wait().unwrap();

    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    }
}
