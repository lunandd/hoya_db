use std::{collections::BTreeMap, fmt::Display};

use std::fmt;

pub type FunctionEnvironment<'a> = BTreeMap<String, Vec<LangType<'a>>>;

#[derive(Debug, Clone, Default)]
pub enum LangType<'a> {
    Number,
    Float,
    Boolean,
    Text,
    List,
    #[default]
    Any,
    Application(&'a str, Vec<LangType<'a>>, Box<&'a LangType<'a>>),
}

impl Display for LangType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LangType::Number => write!(f, "Number"),
            LangType::Float => write!(f, "Float"),
            LangType::Boolean => write!(f, "Boolean"),
            LangType::Text => write!(f, "Text"),
            LangType::List => write!(f, "List"),
            LangType::Any => write!(f, "Any"),
            LangType::Application(_, _, _) => write!(f, "Application"),
        }
    }
}

impl<'a> PartialEq for LangType<'a> {
    fn eq(&self, other: &Self) -> bool {
        use LangType::*;
        match (self, other) {
            (Any, Any) => true,
            (Any, _) => true,
            (_, Any) => true,
            (Number, Number) => true,
            (Float, Float) => true,
            (Boolean, Boolean) => true,
            (Text, Text) => true,
            (List, List) => true,
            (Application(a, b, c), Application(d, e, f)) => a == d && b == e && c == f,
            _ => false,
        }
    }
}
impl<'a> From<&'a LangType<'a>> for LangType<'a> {
    fn from<'b>(l: &'b LangType) -> LangType<'b> {
        match l {
            LangType::Number => LangType::Number,
            LangType::Float => LangType::Float,
            LangType::Boolean => LangType::Boolean,
            LangType::Text => LangType::Text,
            LangType::List => LangType::List,
            LangType::Any => LangType::Any,
            LangType::Application(name, args, ret) => {
                LangType::Application(name, args.to_owned(), ret.to_owned())
            }
        }
    }
}
