use super::object::Object;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment<'ident> {
  records: HashMap<&'ident str, Object>,
}

impl<'ident> Environment<'ident> {
  pub fn new() -> Self {
    Environment {
      records: HashMap::new(),
    }
  }

  pub fn get(&self, name: &'ident str) -> Option<Object> {
    self.records.get(name).map(|val| val.to_owned())
  }

  pub fn set(&mut self, name: &'ident str, value: Object) -> Option<Object> {
    self.records.insert(name, value)
  }
}
