extern crate chrono;
extern crate getopts;

use chrono::prelude::*;
use std::{env, process};
use getopts::Options;

fn print_usage(program: &str, opts: &Options) -> () {
  let brief = format!("Usage: {} [OPTION] [+FORMAT]", program);
  print!("{}", opts.usage(&brief));
  process::exit(0);
}

fn parse_args() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("h", "help", "print this message");

  let matches = opts.parse(&args[1..])
    .unwrap_or_else(|e| panic!(e.to_string()));

  if matches.opt_present("h") {
    print_usage(&program, &opts);
  }
}

fn main() {
  parse_args();
  println!("{}", Local::now().format("%a %b %e %T %Z %Y").to_string());
}
