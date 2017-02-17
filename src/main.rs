extern crate chrono;
extern crate getopts;

use chrono::prelude::*;
use std::{env, process};
use getopts::Options;

#[derive(Debug)]
struct Args {
  utc: bool,
}

fn print_usage(program: &str, opts: &Options) -> () {
  let brief = format!("Usage: {} [OPTION] [+FORMAT]", program);
  print!("{}", opts.usage(&brief));
  process::exit(0);
}

fn parse_args() -> Args {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("u", "utc", "print or set Coordinated Universal Time (UTC)");
  opts.optflag("h", "help", "print this message");

  let matches = opts.parse(&args[1..])
    .unwrap_or_else(|e| panic!(e.to_string()));

  if matches.opt_present("h") {
    print_usage(&program, &opts);
  }

  Args {
    utc: matches.opt_present("u"),
  }
}

fn main() {
  let args = parse_args();

  if args.utc {
    println!("{}", UTC::now().format("%a %b %e %T %Z %Y").to_string());
  } else {
    println!("{}", Local::now().format("%a %b %e %T %Z %Y").to_string());
  }
}
