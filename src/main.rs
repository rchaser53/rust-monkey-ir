extern crate clap;
extern crate reqwest;

mod cli_to_read_file;
use cli_to_read_file::*;

fn main() {
  let body = reqwest::get("https://api.binance.com/api/v1/ticker/24hr?symbol=XRPBTC").unwrap()
                      .text().unwrap();

  let json = convert_str_json(&body).unwrap();
  println!("{}", json);
}



