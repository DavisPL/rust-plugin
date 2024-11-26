#![feature(rustc_private)]

extern crate rustc_driver;
use rustc_driver::{Callbacks, RunCompiler};
use std::env;

struct EmptyCallback;

impl Callbacks for EmptyCallback {}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let _result = RunCompiler::new(&args, &mut EmptyCallback).run();
}
