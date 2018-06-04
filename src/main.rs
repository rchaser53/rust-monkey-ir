extern crate clap;
extern crate serde_json;

use clap::{Arg, App, SubCommand};
use serde_json::{Value, Error};
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.to_string())
}

fn main() {
  let matcher = App::new("input something")
                  .version("1.0")
                  .arg(Arg::with_name("INPUT")
                      .required(true)
                      .index(1))
                  .get_matches();

  let mut filename = "";
  if let Some(c) = matcher.value_of("INPUT") {
    filename = c;
  } else {
    panic!("{} is not found!", filename);
  }

  println!("{}", read_file(&filename).unwrap());
}

