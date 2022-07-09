use super::types::{FunctionEnvironment, LangType};
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

    pub fn return_type_of(&self, name: &str) -> Option<&LangType> {
        match self.env.get(name) {
            Some(types) => Some(types.last().unwrap()),
            None => None,
        }
    }

    pub fn param_types_of(&self, name: &str) -> Vec<LangType> {
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

    pub fn function_type_of<'a>(&'a self, name: &'a str) -> Option<LangType> {
        match (self.param_types_of(name), self.return_type_of(name)) {
            (t, Some(r)) => Some(LangType::Application(name, t, Box::new(r))),
            _ => None,
        }
    }
}

impl Default for Environment<'_> {
    fn default() -> Self {
        let default_env = BTreeMap::from([
            (
                "put".into(),
                vec![LangType::Any, LangType::Text, LangType::Any],
            ),
            ("get".into(), vec![LangType::Text, LangType::Any]),
            ("exists".into(), vec![LangType::Text, LangType::Boolean]),
            ("remove".into(), vec![LangType::Text, LangType::Any]),
            ("store".into(), vec![LangType::Text, LangType::Any]),
        ]);

        Environment { env: default_env }
    }
}
