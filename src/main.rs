extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

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
 
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsWrapper {
    #[serde(with = "items")]
    pub items: HashMap<i64, Items>,
}
 
#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    id: i64,
    info: String,
}

mod items {
    use super::Items;

    use std::collections::HashMap;

    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};

    pub fn serialize<S>(map: &HashMap<i64, Items>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<i64, Items>, D::Error>
        where D: Deserializer<'de>
    {
        let mut map = HashMap::new();
        for item in Vec::<Items>::deserialize(deserializer)? {
            map.insert(item.id, item);
        }
        Ok(map)
    }
}

fn main() {
  // println!("{}", attach_slash_initials("abc/def/"));
  // println!("{}", attach_slash_initials("/abc/def/"));
  // println!("{:?}", JSON_HASH["abc"]);

  // for a in JSON_HASH["abc"].as_array().unwrap() {
  //   println!("{:?}", a);
  // }


    let j = r#" {
                  "items": [
                    {
                      "id": 3,
                      "info": "three"
                    },
                    {
                      "id": 2,
                      "info": "two"
                    }
                  ]
                } "#;

    // println!("{:#?}", serde_json::from_str::<ItemsWrapper>(j).unwrap());

    let json_obj = serde_json::from_str::<ItemsWrapper>(j).unwrap();

    for key in json_obj.items.keys() {
        println!("{:?}", json_obj.items[key]);
    }

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