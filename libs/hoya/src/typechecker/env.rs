use super::types::{FunctionEnvironment, InternalType};
use std::collections::BTreeMap;

pub(crate) fn builtins<'a>() -> [(String, Vec<InternalType<'a>>); 7] {
    [
        (
            String::from("put"),
            vec![InternalType::Any, InternalType::Text, InternalType::Any],
        ),
        (
            String::from("get"),
            vec![InternalType::Text, InternalType::Any],
        ),
        (
            String::from("exists"),
            vec![InternalType::Text, InternalType::Boolean],
        ),
        (
            String::from("remove"),
            vec![InternalType::Text, InternalType::Any],
        ),
        (
            String::from("store"),
            vec![InternalType::Text, InternalType::Unit],
        ),
        (
            String::from("write"),
            vec![InternalType::Text, InternalType::Unit],
        ),
        (
            String::from("writeln"),
            vec![InternalType::Text, InternalType::Unit],
        ),
    ]
}
#[derive(Debug)]
pub struct Environment<'a> {
    env: FunctionEnvironment<'a>,
}

impl Environment<'_> {
    pub fn new<const N: usize>(functions: [(String, Vec<InternalType<'_>>); N]) -> Environment {
        Environment {
            env: BTreeMap::from(functions),
        }
    }

    pub fn builtin() -> Self {
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

    pub fn is_builtin<'a>(&'a self, name: &'a str) -> bool {
        Environment::default().is_defined(name)
    }
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Environment {
            env: BTreeMap::from(builtins()),
        }
    }
}
