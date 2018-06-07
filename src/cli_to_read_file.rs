extern crate serde_json;

use clap::{Arg, App};

use std;
use std::fs::File;
use std::io::prelude::*;
use self::serde_json::{Value, Error};

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.to_string())
}

fn file_content_error_handling(err: std::io::Error) -> String {
  println!("{}", err);
  println!("use '{{}}' insteadof input file.");
  "{}".to_string()
}

pub fn convert_str_json(input_str: &str) -> Result<Value, Error> {
  serde_json::from_str(&input_str)
}

pub fn read_file_from_cli() {
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

  let file_content_str = read_file(&filename).unwrap_or_else(file_content_error_handling);
  let json_obj: Value = match convert_str_json(&file_content_str) {
      Ok(n) => n,
      Err(err) => panic!(err)
  };

  println!("{}", json_obj["key"]);
}