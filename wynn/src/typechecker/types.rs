use std::{collections::BTreeMap, fmt::Display};

use std::fmt;

pub(crate) type FunctionEnvironment<'a> = BTreeMap<String, Vec<InternalType<'a>>>;

// Should probably change it's name to InternalType
#[derive(Debug, Clone)]
pub enum InternalType<'a> {
    Number,
    Float,
    Boolean,
    Text,
    List,
    Any,
    Unit,
    Application(&'a str, Vec<InternalType<'a>>, Box<&'a InternalType<'a>>),
}

impl Default for InternalType<'_> {
    fn default() -> Self {
        Self::Any
    }
}

impl Display for InternalType<'_> {
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

impl<'a> PartialEq for InternalType<'a> {
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
impl<'a> From<&'a InternalType<'a>> for InternalType<'a> {
    fn from<'b>(l: &'b InternalType) -> InternalType<'b> {
        match l {
            InternalType::Number => InternalType::Number,
            InternalType::Float => InternalType::Float,
            InternalType::Boolean => InternalType::Boolean,
            InternalType::Text => InternalType::Text,
            InternalType::List => InternalType::List,
            InternalType::Any => InternalType::Any,
            InternalType::Unit => InternalType::Unit,
            InternalType::Application(name, args, ret) => {
                InternalType::Application(name, args.to_owned(), ret.to_owned())
            }
        }
    }
}
