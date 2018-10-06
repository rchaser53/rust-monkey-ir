use std::collections::HashMap;

use evalute::object::*;

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        store.insert("Null".to_string(), Object::Null);

        Environment {
            store: store,
        }
    }

    pub fn get(&self, name: &str) -> Object {
        if let Some(obj) = self.store.get(name) {
            return obj.clone();
        };
        Object::Error(format!("{} is not found", name))
    }

    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }
}
