extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

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

// fn main() {
//     let j = r#" {
//                   "items": [
//                     {
//                       "id": 3,
//                       "info": "three"
//                     },
//                     {
//                       "id": 2,
//                       "info": "two"
//                     }
//                   ]
//                 } "#;

    // println!("{:#?}", serde_json::from_str::<ItemsWrapper>(j).unwrap());
//     let json_obj = serde_json::from_str::<ItemsWrapper>(j).unwrap();

    // for key in json_obj.items.keys() {
    //     println!("{:?}", json_obj.items[key]);
    // }
// }