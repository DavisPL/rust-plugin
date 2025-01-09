use std::fs::File;
use std::path::Path;
use std::{env, fs};

use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};

#[derive(Debug)]
pub enum CrateKind {
    Bin,
    Lib,
}

/// Arguments passed to `rustc` from Cargo.
#[derive(Debug)]
pub struct Args {
    /// Name of the crate being compiled.
    name: Option<String>,

    /// The kind of this crate.
    kind: Option<CrateKind>,

    /// Whether this crate is being compiled for release (`true`) or debugging (`false`).
    is_release: bool,

    /// Whether this crate is being compiled for a test (`true`) or not (`false`).
    is_test: bool,
}

impl Args {
    pub fn from_raw(args: &[String]) -> Self {
        let mut name = None;
        let mut kind = None;
        let mut is_release = false;
        let mut is_test = false;

        for pair in args.windows(2) {
            let a1 = pair[0].as_str();
            let a2 = pair[1].as_str();

            match (a1, a2) {
                ("--crate-name", a2) => name = Some(a2.to_owned()),
                ("--crate-type", "bin") => kind = Some(CrateKind::Bin),
                ("--crate-type", _) => {
                    // TODO: Account for the other crate types
                    kind = Some(CrateKind::Lib)
                }
                _ => {}
            }

            if a1 == "--release" || a2 == "--release" {
                is_release = true;
            }

            if a1 == "--test" || a2 == "test" {
                is_test = true;
            }
        }

        Args {
            name,
            kind,
            is_release,
            is_test,
        }
    }

    pub fn name_or(&self, default: &str) -> String {
        self.name.clone().unwrap_or(default.to_owned())
    }
}

pub fn initialize_logging(args: &Args) {
    let log_level = env::var("PLUGIN_LOG_LEVEL").unwrap_or_else(|_| "OFF".to_string());

    let level_filter = match log_level.as_str() {
        "DEBUG" => LevelFilter::Debug,
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        _ => LevelFilter::Off,
    };

    if level_filter == LevelFilter::Off {
        return;
    }

    // Determine the root directory
    let root_dir = env!("CARGO_MANIFEST_DIR");

    // Create logging directory if it doesn't already exist
    let log_dir = Path::new(root_dir).join(".log");
    fs::create_dir_all(&log_dir).expect(&format!(
        "Failed to create plugin logging directory at {}",
        log_dir.display()
    ));

    // Create logging file
    let path = log_dir.join(args.name_or("-")).with_extension("log");
    let log = File::create(&path).expect(&format!(
        "Failed to create plugin logging file at {}",
        path.display()
    ));

    let config = ConfigBuilder::new()
        .set_time_level(log::LevelFilter::Off) // Don't show time
        .set_target_level(log::LevelFilter::Off) // Don't show target (like "(1)")
        .set_thread_level(log::LevelFilter::Off) // Don't show thread ID
        .set_location_level(log::LevelFilter::Off) // Don't show file/line location
        .build();

    WriteLogger::init(simplelog::LevelFilter::Debug, config, log)
        .expect("Failed to initialize plugin logger");
}
