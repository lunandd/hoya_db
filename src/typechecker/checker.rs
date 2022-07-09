use crate::parser::ast::Expr;

use super::env::Environment;
use super::errors::TypeCheckerError;
use super::types::LangType;

#[derive(Debug)]
pub struct Typechecker<'a> {
    pub env: Environment<'a>,
}

impl Default for Typechecker<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Typechecker<'_> {
    pub fn new() -> Self {
        Typechecker {
            env: Environment::default(),
        }
    }

    // Let's hope I'm implementing Bidirectional typechecking correctly
    pub fn synthesize<'a>(&'a self, ast: &'a Expr) -> Result<LangType, TypeCheckerError> {
        match ast {
            Expr::Text(_) => Ok(LangType::Text),
            Expr::List(_) => Ok(LangType::List),
            Expr::Float(_) => Ok(LangType::Float),
            Expr::Number(_) => Ok(LangType::Number),
            Expr::Boolean(_) => Ok(LangType::Boolean),
            Expr::Identifier(name) => {
                if let Some(ret) = self.env.return_type_of(name) {
                    Ok(LangType::Application(name, vec![], Box::new(ret)))
                } else {
                    Err(TypeCheckerError::FunctionNotFound(name.into()))
                }
            }
            Expr::Call(name, args) => {
                let str_name = match &**name {
                    Expr::Identifier(f) => f,
                    _ => unreachable!(),
                };
                if let Some(ret) = self.env.return_type_of(str_name) {
                    Ok(LangType::Application(
                        str_name,
                        args.iter()
                            // TODO: Check if synthesized results are Ok
                            // i.e when ```put a "a"``` is executed on the default Environment it
                            // fails because a is not defined and it's value is
                            // FunctionNotFound("a")
                            // Basically return a Vec<Result<LangType, TypeCheckerError>>
                            .map(|e| self.synthesize(e).unwrap())
                            .collect::<Vec<_>>(),
                        Box::new(ret),
                    ))
                } else {
                    Err(TypeCheckerError::FunctionNotFound(str_name.to_string()))
                }
            }
        }
    }

    pub fn check<'a>(
        &'a self,
        expected: &'a LangType,
        ast: &'a Expr,
    ) -> Result<(), TypeCheckerError> {
        let synthesized = self.synthesize(ast);
        match synthesized {
            Ok(internal_type) => match internal_type {
                LangType::Application(name, ref arg_types, _) => {
                    let expected = self.env.param_types_of(name);

                    if expected == *arg_types {
                        Ok(())
                    } else {
                        Err(TypeCheckerError::InvalidTypesFound {
                            expected,
                            found: arg_types.to_owned(),
                        })
                    }
                }
                found => {
                    if &found == expected {
                        Ok(())
                    } else {
                        Err(TypeCheckerError::InvalidTypeFound {
                            expected: expected.into(),
                            found,
                        })
                    }
                }
            },
            Err(err) => Err(err),
        }
    }
}
