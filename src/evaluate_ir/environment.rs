use std::collections::HashMap;

use evaluate_ir::object::*;
use parser::expressions::*;

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        store.insert("Null".to_string(), Object::Null);
        store.insert("printf".to_string(), Object::BuildIn(BuildIn::Printf));

        Environment { store: store }
    }

    pub fn get(&self, name: &str, location: Location) -> Object {
        if let Some(obj) = self.store.get(name) {
            return obj.clone();
        };
        Object::Error(format!("{} is not found. row: {}", name, location.row))
    }

    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }
}
