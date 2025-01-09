use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let mut command = Command::new("cargo")
        .arg("check")
        .env("RUSTC_WRAPPER", "plugin-driver")
        .env("PLUGIN_LOG_LEVEL", "DEBUG")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

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
        eprintln!("Failed with exit code: {:?}", status.code());
    }
}
