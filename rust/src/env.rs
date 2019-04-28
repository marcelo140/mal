use std::collections::HashMap;

use crate::types::*;

pub struct Env<'a> {
    mappings: HashMap<String, MalVal>,
    outer: Option<&'a Env<'a>>, 
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>) -> Self {
        Env {
            mappings: HashMap::new(),
            outer: outer,
        }
    }

    pub fn get(&self, key: &str) -> Option<MalVal> {
        match self.mappings.get(key) {
            Some(v) => Some(v.clone()),
            None => match &self.outer {
                Some(env) => env.get(key),
                None => None,
            }
        }
    }

    pub fn set(&mut self, key: String, value: MalVal) {
        self.mappings.insert(key, value);
    }
}
