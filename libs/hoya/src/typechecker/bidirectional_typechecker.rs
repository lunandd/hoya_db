use crate::parser::ast::Ast;

use super::env::Environment;
use super::errors::TypeCheckerError;
use super::types::InternalType;

#[derive(Debug, Default)]
pub struct Typechecker {
    pub env: Environment,
}

impl Typechecker {
    pub fn new(env: Environment) -> Typechecker {
        Typechecker { env }
    }

    pub fn synthesize(&self, ast: &Ast) -> Result<InternalType, TypeCheckerError> {
        match ast {
            Ast::Text(_) => Ok(InternalType::Text),
            Ast::List(_) => Ok(InternalType::List),
            Ast::Float(_) => Ok(InternalType::Float),
            Ast::Number(_) => Ok(InternalType::Number),
            Ast::Boolean(_) => Ok(InternalType::Boolean),
            Ast::Unit(_) => Ok(InternalType::Unit),
            Ast::Identifier(name) => {
                if let Some(ret) = self.env.return_type_of(name) {
                    Ok(InternalType::Application(
                        name.into(),
                        vec![],
                        Box::new(ret.to_owned()),
                    ))
                } else {
                    Err(TypeCheckerError::FunctionNotFound(name.into()))
                }
            }
            Ast::Call(name, args) => {
                let str_name = match &**name {
                    Ast::Identifier(f) => f,
                    _ => unreachable!(),
                };
                if let Some(ret) = self.env.return_type_of(str_name) {
                    /* TODO: Check if synthesized results are Ok
                    i.e when ```(put a "a")``` is executed on the default Environment
                    it fails because a is not defined. Basically return
                    a ```Vector``` instead of just a ```Result```*/
                    Ok(InternalType::Application(
                        str_name.into(),
                        args.iter()
                            .map(|e| self.synthesize(e).unwrap())
                            .collect::<Vec<_>>(),
                        Box::new(ret.into()),
                    ))
                } else {
                    Err(TypeCheckerError::FunctionNotFound(str_name.to_string()))
                }
            }
        }
    }

    pub fn check<'a>(
        &'a self,
        expected: &'a InternalType,
        ast: &'a Ast,
    ) -> Result<(), TypeCheckerError> {
        let synthesized = self.synthesize(ast);
        match synthesized {
            Ok(internal_type) => match internal_type {
                InternalType::Application(name, ref arg_types, _) => {
                    let expected = self.env.param_types_of(&name[..]);

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
