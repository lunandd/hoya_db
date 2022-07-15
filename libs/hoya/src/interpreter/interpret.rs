use ariadne::{Label, Report, ReportKind, Source};
use combine::ParseError;
use std::io::{self, Write};
use std::{ops::Deref, rc::Rc};

use db::{DBTypes, Database};

use super::types::InterpreterValue;
use crate::parser::parse;
use crate::typechecker::types::InternalType;
use crate::{interpreter::env::Environment, parser::ast::Ast};

pub struct Interpreter {
    env: Environment,
    db: Database,
}

impl Interpreter {
    pub fn new(db: Database, env: Environment) -> Interpreter {
        Interpreter { env, db }
    }

    fn text_ast_to_string(&self, ast: &Ast) -> String {
        match ast {
            Ast::Text(t) => t.to_string(),
            _ => unreachable!(),
        }
    }

    fn stringify(&self, val: &InterpreterValue) -> String {
        match val {
            InterpreterValue::Text(s) => format!("{}", *s),
            InterpreterValue::Boolean(b) => (*b).to_string(),
            InterpreterValue::List(l) => format!("{:?}", *l),
            InterpreterValue::Number(n) => (*n).to_string(),
            InterpreterValue::Float(f) => (*f).to_string(),
            InterpreterValue::Unit(_) => "()".to_string(),
            InterpreterValue::Identifier(i) => format!("{}", *i),
            InterpreterValue::Call(i, a) => format!(
                "({} {:?})",
                self.stringify(i.deref()),
                a.deref()
                    .iter()
                    .map(|a| self.stringify(a))
                    .collect::<Vec<String>>()
            ),
        }
    }

    fn eval_builtin(&self, ast: &Ast) -> InterpreterValue {
        match ast {
            Ast::Call(name, args) => {
                if let Ast::Identifier(identifier) = &**name {
                    match &identifier[..] {
                        "write" => {
                            let mut stdout = io::stdout();
                            stdout
                                .write_all(self.stringify(&self.eval_ast(&args[0])).as_bytes())
                                .unwrap();
                            InterpreterValue::Unit(Rc::new(()))
                        }
                        "writeln" => {
                            let mut stdout = io::stdout();
                            stdout
                                .write_all(
                                    (self.stringify(&self.eval_ast(&args[0])) + "\n").as_bytes(),
                                )
                                .unwrap();
                            InterpreterValue::Unit(Rc::new(()))
                        }
                        "put" => self
                            .db
                            .put(
                                self.text_ast_to_string(&args[1]),
                                DBTypes::from(self.eval_ast(&args[0])),
                            )
                            .into(),
                        "get" => self.db.get(&self.text_ast_to_string(&args[0])).into(),
                        "exists" => InterpreterValue::Boolean(Rc::new(
                            self.db.exists(&self.text_ast_to_string(&args[0])),
                        )),
                        "remove" => self.db.remove(&self.text_ast_to_string(&args[0])).into(),
                        "store" => todo!(),
                        _ => todo!(),
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn eval_ast(&self, ast: &Ast) -> InterpreterValue {
        match ast {
            Ast::Identifier(..) => todo!("No definable functions yet"),
            Ast::Call(..) => {
                // Temporarily calling eval_builtin until I implement the def statement
                // Example: (def greet (name: Text) -> Text
                //              (writeln (concat "Hello " name "!")))
                self.eval_builtin(ast)
            }
            e => e.to_owned().into(),
        }
    }

    pub fn interpret(&self, code: &str) {
        let parser_result = &parse(code).map(|x| x.0);

        if let Ok(parsed) = parser_result {
            let checked = self.env.typechecker.check(&InternalType::Any, parsed);

            match checked {
                Ok(_) => {
                    println!("{}", self.stringify(&self.eval_ast(parsed)));
                }
                Err(ref e) => {
                    Report::build(ReportKind::Error, (), 1)
                        .with_message(e.to_short_error().to_string())
                        .with_label(Label::new(0..1))
                        .with_label(Label::new(0..1).with_message(format!("{e}")))
                        .finish()
                        .print(Source::from(code))
                        .unwrap();
                }
            }
        } else {
            let err = match parser_result {
                Err(e) => e,
                _ => unreachable!(),
            };
            Report::build(ReportKind::Error, (), 1)
                .with_message({
                    let formatted_error = format!("{}", err);
                    let err_vec = formatted_error.split('\n').skip(1).collect::<Vec<_>>();
                    format!("{}\n{}", err_vec[0], err_vec[1])
                })
                .with_label(
                    Label::new(err.position().line as usize..err.position().column as usize)
                        .with_message(err.errors.first().unwrap()),
                )
                .finish()
                .print(Source::from(code))
                .unwrap();
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            env: Environment::builtin(),
            db: Database::default(),
        }
    }
}
