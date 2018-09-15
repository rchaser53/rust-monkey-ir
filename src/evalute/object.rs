#[derive(Debug, Clone)]
pub enum ObjectType {
  IntegerObj,
  BooleanObj,
  NullObj,
}

pub trait Object {
  fn inspect(&self) -> String;
  fn object_type(&self) -> ObjectType;
}


#[derive(Debug, Clone)]
pub struct Integer {
  pub value: i64
}
impl Object for Integer {
  fn inspect(&self) -> String {
    self.value.to_string()
  }

  fn object_type(&self) -> ObjectType {
    ObjectType::IntegerObj
  }
}


#[derive(Debug, Clone)]
pub struct Boolean {
  pub value: bool
}
impl Object for Boolean {
  fn inspect(&self) -> String {
    self.value.to_string()
  }

  fn object_type(&self) -> ObjectType {
    ObjectType::BooleanObj
  }
}


#[derive(Debug, Clone)]
pub struct Null {}
impl Object for Null {
  fn inspect(&self) -> String {
    "null".to_string()
  }

  fn object_type(&self) -> ObjectType {
    ObjectType::NullObj
  }
}