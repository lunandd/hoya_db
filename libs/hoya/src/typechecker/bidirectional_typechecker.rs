use crate::parser::ast::Expr;

use super::env::Environment;
use super::errors::TypeCheckerError;
use super::types::InternalType;

#[derive(Debug, Default)]
pub struct Typechecker<'a> {
    pub env: Environment<'a>,
}

impl Typechecker<'_> {
    pub fn new(env: Environment<'_>) -> Typechecker<'_> {
        Typechecker { env }
    }

    // Let's hope I'm implementing Bidirectional typechecking correctly
    pub fn synthesize<'a>(&'a self, ast: &'a Expr) -> Result<InternalType, TypeCheckerError> {
        match ast {
            Expr::Text(_) => Ok(InternalType::Text),
            Expr::List(_) => Ok(InternalType::List),
            Expr::Float(_) => Ok(InternalType::Float),
            Expr::Number(_) => Ok(InternalType::Number),
            Expr::Boolean(_) => Ok(InternalType::Boolean),
            Expr::Unit(_) => Ok(InternalType::Unit),
            Expr::Identifier(name) => {
                if let Some(ret) = self.env.return_type_of(name) {
                    Ok(InternalType::Application(name, vec![], Box::new(ret)))
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
                    Ok(InternalType::Application(
                        str_name,
                        args.iter()
                            // TODO Check if synthesized results are Ok
                            // i.e when ```put a "a"``` is executed on the default Environment it
                            // fails because a is not defined and it's value is
                            // FunctionNotFound("a")
                            // Basically return a Vec<Result<InternalType, TypeCheckerError>>
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
        expected: &'a InternalType,
        ast: &'a Expr,
    ) -> Result<(), TypeCheckerError> {
        let synthesized = self.synthesize(ast);
        match synthesized {
            Ok(internal_type) => match internal_type {
                InternalType::Application(name, ref arg_types, _) => {
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
