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

    fn single_result_synthesize<'a>(
        &'a self,
        ast: &'a Expr,
    ) -> Result<InternalType, TypeCheckerError> {
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
            _ => unreachable!(),
        }
    }

    pub fn synthesize<'a>(&'a self, ast: &'a Expr) -> Result<InternalType, Vec<TypeCheckerError>> {
        match ast {
            Expr::Call(name, args) => {
                let str_name = match &**name {
                    Expr::Identifier(f) => f,
                    _ => unreachable!(),
                };
                if let Some(ret) = self.env.return_type_of(str_name) {
                    let application_args =
                        args.iter().map(|e| self.synthesize(e)).collect::<Vec<_>>();

                    let contains_errors = application_args
                        .iter()
                        .map(|arg| arg.is_err())
                        .all(|arg| arg);

                    if !contains_errors {
                        Ok(InternalType::Application(
                            str_name,
                            application_args
                                .into_iter()
                                .map(|arg| arg.unwrap())
                                .collect::<Vec<_>>(),
                            Box::new(ret),
                        ))
                    } else {
                        Err(application_args
                            .into_iter()
                            .map(|arg| match arg {
                                Err(e) => e,
                                _ => unreachable!(),
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                            .flatten()
                            .collect())
                    }
                } else {
                    Err(vec![TypeCheckerError::FunctionNotFound(
                        str_name.to_string(),
                    )])
                }
            }
            single_res => self
                .single_result_synthesize(single_res)
                .map_err(|err| vec![err]),
        }
    }

    pub fn check<'a>(
        &'a self,
        expected: &'a InternalType,
        ast: &'a Expr,
    ) -> Result<(), Vec<TypeCheckerError>> {
        let synthesized = self.synthesize(ast);
        match synthesized {
            Ok(internal_type) => match internal_type {
                InternalType::Application(name, ref arg_types, _) => {
                    let expected = self.env.param_types_of(name);

                    if expected == *arg_types {
                        Ok(())
                    } else {
                        Err(vec![TypeCheckerError::InvalidTypesFound {
                            expected,
                            found: arg_types.to_owned(),
                        }])
                    }
                }
                found => {
                    if &found == expected {
                        Ok(())
                    } else {
                        Err(vec![TypeCheckerError::InvalidTypeFound {
                            expected: expected.into(),
                            found,
                        }])
                    }
                }
            },
            Err(err) => Err(err),
        }
    }
}
