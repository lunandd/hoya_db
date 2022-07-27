use std::collections::BTreeMap;

use crate::{
    parser::ast::Ast,
    typechecker::{bidirectional_typechecker::Typechecker, types::InternalType},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("Function {0} already defined")]
    FunctionAlreadyDefined(String),
}

#[derive(Default)]
pub struct Environment {
    pub typechecker: Typechecker,
    pub defined_functions: BTreeMap<String, Ast>,
}

impl Environment {
    pub fn builtin() -> Self {
        Self::default()
    }

    pub fn is_builtin(&self, name: &'_ str) -> bool {
        self.typechecker.env.is_builtin(name)
    }

    pub fn is_defined(&self, name: &'_ str) -> bool {
        self.defined_functions.contains_key(name) && self.typechecker.env.is_defined(name)
    }

    pub fn define_function(
        &mut self,
        name: String,
        expr: Ast,
        types: Vec<InternalType>,
    ) -> Result<(), EnvironmentError> {
        if self.is_defined(&name) {
            Err(EnvironmentError::FunctionAlreadyDefined(name))
        } else {
            self.typechecker.env.define(name.to_owned(), types);
            self.defined_functions.insert(name, expr);
            Ok(())
        }
    }
}
