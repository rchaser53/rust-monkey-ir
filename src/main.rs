extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde_json::{Value, Error};

use std::collections::HashMap;

mod cli_to_read_file;
use cli_to_read_file::*;

lazy_static! {
    pub static ref JSON_HASH: HashMap<&'static str, Value> = {
      let mut m = HashMap::new();
      let val: Value = serde_json::from_str(r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#).unwrap();

      m.insert("abc", val);
      m
    };
}

fn main() {
  println!("{}", attach_slash_initials("abc/def/"));
  println!("{}", attach_slash_initials("/abc/def/"));
}

fn attach_slash_initials(target_str: &str) -> String {
  let re = Regex::new(r"^/").unwrap();

  if re.is_match(target_str) {
    return target_str.to_string();
  }
  return "/".to_owned() + target_str;
}

  // let separeteds: Vec<&str> = url_string.split("/").collect();
  // for s in separeteds {
  //   println!("{}", s);
  // }

 // let body = reqwest::get("https://api.binance.com/api/v1/ticker/24hr?symbol=XRPBTC").unwrap()
  //                     .text().unwrap();

  // let json = convert_str_json(&body).unwrap();
  // println!("{}", json);