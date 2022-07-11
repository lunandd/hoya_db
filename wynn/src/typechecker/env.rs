use super::types::{FunctionEnvironment, InternalType};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Environment<'a> {
    env: FunctionEnvironment<'a>,
}

impl Environment<'_> {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.env.contains_key(name)
    }

    pub fn return_type_of(&self, name: &str) -> Option<&InternalType> {
        match self.env.get(name) {
            Some(types) => Some(types.last().unwrap()),
            None => None,
        }
    }

    pub fn param_types_of(&self, name: &str) -> Vec<InternalType> {
        match self.env.get(name) {
            Some(types) => {
                if types.len() == 1 {
                    vec![]
                } else {
                    types[0..types.len() - 1].to_vec()
                }
            }
            None => vec![],
        }
    }

    pub fn function_type_of<'a>(&'a self, name: &'a str) -> Option<InternalType> {
        match (self.param_types_of(name), self.return_type_of(name)) {
            (t, Some(r)) => Some(InternalType::Application(name, t, Box::new(r))),
            _ => None,
        }
    }
}

impl Default for Environment<'_> {
    fn default() -> Self {
        let default_env = BTreeMap::from([
            (
                "put".into(),
                vec![InternalType::Any, InternalType::Text, InternalType::Any],
            ),
            ("get".into(), vec![InternalType::Text, InternalType::Any]),
            (
                "exists".into(),
                vec![InternalType::Text, InternalType::Boolean],
            ),
            ("remove".into(), vec![InternalType::Text, InternalType::Any]),
            // Stores database to file
            ("store".into(), vec![InternalType::Text, InternalType::Unit]),
            ("print".into(), vec![InternalType::Text, InternalType::Unit]),
        ]);

        Environment { env: default_env }
    }
}
