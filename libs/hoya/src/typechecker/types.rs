use std::collections::HashMap;
use std::fmt::Display;

use std::fmt;

pub(crate) type FunctionEnvironment = HashMap<String, Vec<InternalType>>;

#[derive(Debug, Clone)]
pub enum InternalType {
    Number,
    Float,
    Boolean,
    Text,
    List,
    Any,
    Unit,
    Application(String, Vec<InternalType>, Box<InternalType>),
}

impl Default for InternalType {
    fn default() -> Self {
        Self::Any
    }
}

impl Display for InternalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalType::Number => write!(f, "Number"),
            InternalType::Float => write!(f, "Float"),
            InternalType::Boolean => write!(f, "Boolean"),
            InternalType::Text => write!(f, "Text"),
            InternalType::List => write!(f, "List"),
            InternalType::Any => write!(f, "Any"),
            InternalType::Unit => write!(f, "Unit"),
            InternalType::Application(_, _, _) => write!(f, "Application"),
        }
    }
}

impl PartialEq for InternalType {
    fn eq(&self, other: &Self) -> bool {
        use InternalType::*;
        match (self, other) {
            (Any, Any) => true,
            (Any, _) => true,
            (_, Any) => true,
            (Number, Number) => true,
            (Float, Float) => true,
            (Boolean, Boolean) => true,
            (Text, Text) => true,
            (List, List) => true,
            (Unit, Unit) => true,
            (Application(a, b, c), Application(d, e, f)) => a == d && b == e && c == f,
            _ => false,
        }
    }
}
impl From<&InternalType> for InternalType {
    fn from(l: &InternalType) -> InternalType {
        match l {
            InternalType::Number => InternalType::Number,
            InternalType::Float => InternalType::Float,
            InternalType::Boolean => InternalType::Boolean,
            InternalType::Text => InternalType::Text,
            InternalType::List => InternalType::List,
            InternalType::Any => InternalType::Any,
            InternalType::Unit => InternalType::Unit,
            InternalType::Application(name, args, ret) => {
                InternalType::Application(name.to_string(), args.to_owned(), ret.to_owned())
            }
        }
    }
}
