extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
  let hoge = App::new("My Super Program")
                  .version("1.0")
                  .arg(Arg::with_name("INPUT")
                      .help("Sets the input file to use")
                      .required(true)
                      .index(1)
                  )
                  .get_matches();

  let mut ret = "";
  if let Some(c) = hoge.value_of("INPUT") {
    ret = c;
  } else {
    ret = "nothing!";
  }
  println!("{}", ret);
}

