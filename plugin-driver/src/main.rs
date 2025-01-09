#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;

use rustc_driver::{Callbacks, RunCompiler};
use std::env;

mod utils;
use utils::{initialize_logging, Args};

struct EmptyCallback;

impl Callbacks for EmptyCallback {}

fn main() {
    // Collect the arguments passed to us by Cargo
    let raw_args: Vec<String> = env::args().skip(1).collect();

    // Parse the arguments into a nicer form
    let args = Args::from_raw(&raw_args);

    // Initialize logging
    initialize_logging(&args);

    // Run the compiler
    let _result = RunCompiler::new(&raw_args, &mut EmptyCallback).run();
}
