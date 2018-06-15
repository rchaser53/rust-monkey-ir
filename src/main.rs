extern crate clap;
extern crate reqwest;
extern crate regex;
extern crate serde_json;

#[derive(Debug)]
pub enum AstType {
  Start,
  End,
  Normal,
}

#[derive(Debug)]
struct Part {
  kind: AstType,
  value: char,
}

impl Part {
  fn new(kind: AstType , imput: char) -> Part {
    return Part {
      kind: kind,
      value: imput
    }
  }
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }

fn main() {
  let input_str = "{a b  c}";

  let mut i_vec: Vec<Part> = Vec::with_capacity(input_str.len());;
  for cha in input_str.chars() {
    match cha {
      '{' => {
        i_vec.push(Part::new(AstType::Start, cha));
      },
      '}' => {
        i_vec.push(Part::new(AstType::End, cha));
      },
      ' ' => {},
      _ => {
        i_vec.push(Part::new(AstType::Normal, cha));
      }
    };
  }

  println!("{:?}", i_vec);
}

  // let separeteds: Vec<&str> = url_string.split("/").collect();
  // for s in separeteds {
  //   println!("{}", s);
  // }
// fn main() {
  // let client = reqwest::Client::new();
  // let status = client
  //     .request(reqwest::Method::Options, "http://localhost:3000/test")
  //     .send()
  //     .map(|res| res.status())
  //     .map_err(|err| panic!(err));
  
  // if status.unwrap() == reqwest::StatusCode::Ok {
  //   let res = client.put("http://localhost:3000/test")
  //         .body("nyan")
  //         .send();

  //   println!("{:?}", res.unwrap().text());
  // }


  // let json = convert_str_json(&body).unwrap();
  // println!("{:?}", res);
// }
 // let body = reqwest::get("https://api.binance.com/api/v1/ticker/24hr?symbol=XRPBTC").unwrap()
  //                     .text().unwrap();

  // let json = convert_str_json(&body).unwrap();
  // println!("{}", json);