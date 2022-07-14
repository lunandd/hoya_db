use std::rc::Rc;

use db::DBTypes;

use crate::parser::ast::Expr;

#[derive(Debug, Clone)]
pub enum InterpreterValue {
    Number(Rc<isize>),
    Float(Rc<f64>),
    Boolean(Rc<bool>),
    Text(Rc<String>),
    List(Rc<Vec<InterpreterValue>>),
    Identifier(Rc<String>),
    Call(Rc<InterpreterValue>, Rc<Vec<InterpreterValue>>),
    Unit(Rc<()>),
}

impl From<Expr> for InterpreterValue {
    fn from(e: Expr) -> Self {
        match e {
            Expr::Text(t) => Self::Text(Rc::new(t)),
            Expr::List(l) => Self::List(Rc::new(l.into_iter().map(|e| e.into()).collect())),
            Expr::Call(n, a) => Self::Call(
                Rc::new(InterpreterValue::from(*n)),
                Rc::new(a.into_iter().map(|e| e.into()).collect()),
            ),
            Expr::Float(f) => Self::Float(Rc::new(f)),
            Expr::Number(n) => Self::Number(Rc::new(n)),
            Expr::Identifier(i) => Self::Identifier(Rc::new(i)),
            Expr::Boolean(b) => Self::Boolean(Rc::new(b)),
            Expr::Unit(()) => Self::Unit(Rc::new(())),
        }
    }
}

impl From<DBTypes> for InterpreterValue {
    fn from(d: DBTypes) -> Self {
        match d {
            DBTypes::Text(t) => Self::Text(Rc::new(t)),
            DBTypes::Number(n) => Self::Number(Rc::new(n)),
            DBTypes::Float(f) => Self::Float(Rc::new(f)),
            DBTypes::Boolean(b) => Self::Boolean(Rc::new(b)),
            DBTypes::Unit(u) => Self::Unit(Rc::new(u)),
            DBTypes::List(l) => Self::List(Rc::new(l.into_iter().map(|d| d.into()).collect())),
        }
    }
}

impl From<Option<DBTypes>> for InterpreterValue {
    fn from(d: Option<DBTypes>) -> Self {
        match d {
            Some(s) => s.into(),
            None => Self::Unit(Rc::new(())),
        }
    }
}

impl From<InterpreterValue> for DBTypes {
    fn from(i: InterpreterValue) -> Self {
        match i {
            InterpreterValue::Text(t) => Self::Text(t.to_string()),
            InterpreterValue::Number(n) => Self::Number(*n),
            InterpreterValue::Float(f) => Self::Float(*f),
            InterpreterValue::Boolean(b) => Self::Boolean(*b),
            InterpreterValue::Unit(_) => Self::Unit(()),
            InterpreterValue::List(l) => {
                let mut list: Vec<DBTypes> = vec![];
                for i in (*l).clone().into_iter() {
                    list.push(i.into())
                }
                Self::List(list)
            }
            _ => unreachable!(),
        }
    }
}
