use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Integer(int) => write!(f, "{}", int.to_string()),
            Object::Boolean(boolean) => write!(f, "{}", boolean.to_string()),
            Object::Null => write!(f, "Null"),
        }
    }
}
