use ariadne::{Label, Report, ReportKind, Source};
use combine::ParseError;
use std::io::{self, Write};
use std::{ops::Deref, rc::Rc};

use db::{DBTypes, Database};

use super::types::InterpreterValue;
use crate::parser::parse;
use crate::typechecker::bidirectional_typechecker::Typechecker;
use crate::typechecker::types::InternalType;
use crate::{parser::ast::Expr, typechecker::env::Environment};

pub struct Interpreter<'a> {
    typechecker: Typechecker<'a>,
    env: Environment<'a>,
    db: Database,
}

impl Interpreter<'_> {
    pub fn new<'a>(
        db: Database,
        env: Environment<'a>,
        typechecker: Typechecker<'a>,
    ) -> Interpreter<'a> {
        Interpreter {
            typechecker,
            env,
            db,
        }
    }

    fn text_expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Text(t) => t.to_string(),
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

    fn eval_builtin(&self, expr: &Expr) -> InterpreterValue {
        match expr {
            Expr::Call(name, args) => {
                if let Expr::Identifier(identifier) = &**name {
                    match &identifier[..] {
                        "write" => {
                            let mut stdout = io::stdout();
                            stdout
                                .write_all(self.stringify(&self.eval_expr(&args[0])).as_bytes())
                                .unwrap();
                            InterpreterValue::Unit(Rc::new(()))
                        }
                        "writeln" => {
                            let mut stdout = io::stdout();
                            stdout
                                .write_all(
                                    (self.stringify(&self.eval_expr(&args[0])) + "\n").as_bytes(),
                                )
                                .unwrap();
                            InterpreterValue::Unit(Rc::new(()))
                        }
                        "put" => self
                            .db
                            .put(
                                self.text_expr_to_string(&args[1]),
                                DBTypes::from(self.eval_expr(&args[0])),
                            )
                            .into(),
                        "get" => self.db.get(&self.text_expr_to_string(&args[0])).into(),
                        "exists" => InterpreterValue::Boolean(Rc::new(
                            self.db.exists(&self.text_expr_to_string(&args[0])),
                        )),
                        "remove" => self.db.remove(&self.text_expr_to_string(&args[0])).into(),
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

    pub fn eval_expr(&self, expr: &Expr) -> InterpreterValue {
        match expr {
            Expr::Identifier(..) => todo!("No definable functions yet"),
            Expr::Call(..) => {
                // Temporarily calling eval_builtin until I implement the def statement
                // Example: (def greet (name: Text) -> Text
                //              (writeln (concat "Hello " name "!")))
                self.eval_builtin(expr)
            }
            e => e.to_owned().into(),
        }
    }

    pub fn interpret(&self, code: &str) {
        let parser_result = &parse(code).map(|x| x.0);

        if let Ok(parsed) = parser_result {
            let checked = self.typechecker.check(&InternalType::Any, parsed);
            println!("{:#?}", parsed);

            match checked {
                Ok(_) => {
                    println!("{}", self.stringify(&self.eval_expr(parsed)));
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

impl Default for Interpreter<'_> {
    fn default() -> Self {
        Self {
            typechecker: Typechecker::new(Environment::builtin()),
            env: Environment::builtin(),
            db: Database::default(),
        }
    }
}
