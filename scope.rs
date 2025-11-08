use std::collections::HashMap;

#[derive(Default)]
pub struct Scope {
    stack: Vec<HashMap<String, String>>,
}

impl Scope {
    pub fn new() -> Self { Self { stack: vec![HashMap::new()] } } // global
    pub fn push(&mut self) { self.stack.push(HashMap::new()); }
    pub fn pop(&mut self) { self.stack.pop(); }

    pub fn define(&mut self, name: &str, value: String) {
        if let Some(top) = self.stack.last_mut() {
            top.insert(name.to_string(), value);
        }
    }

    pub fn resolve(&self, name: &str) -> Option<String> {
        for m in self.stack.iter().rev() {
            if let Some(v) = m.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}
