extern crate chrono;

use chrono::prelude::*;

fn main() {
  let local: DateTime<Local> = Local::now();
  println!("{}", local.format("%a %b %e %T JST %Y").to_string());
}
