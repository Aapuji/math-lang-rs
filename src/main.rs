use std::env;
use std::process;

use math_lang::Config;

fn main() {
  let config = Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
}