use thiserror::Error;

use super::types::LangType;

#[derive(Error, Debug)]
pub enum TypeCheckerError<'a> {
    #[error("Function `{0}` does not exist")]
    FunctionNotFound(String),
    #[error("Expected type {expected}, but found {found}")]
    InvalidTypeFound {
        expected: LangType<'a>,
        found: LangType<'a>,
    },
    #[error("Expected types {expected:?}, but found {found:?}")]
    InvalidTypesFound {
        expected: Vec<LangType<'a>>,
        found: Vec<LangType<'a>>,
    },
}

#[derive(Error, Debug)]
pub enum ShortTypeCheckerError<'a> {
    #[error("Function `{0}` not found")]
    FunctionNotFound(String),
    #[error("Invalid Type `{found}` Found")]
    InvalidTypeFound {
        expected: LangType<'a>,
        found: LangType<'a>,
    },
    #[error("Invalid Types Found")]
    InvalidTypesFound {
        expected: Vec<LangType<'a>>,
        found: Vec<LangType<'a>>,
    },
}

impl ShortTypeCheckerError<'_> {
    pub fn to_long_error(&'_ self) -> TypeCheckerError<'_> {
        match self {
            Self::FunctionNotFound(f) => TypeCheckerError::FunctionNotFound(f.to_string()),
            Self::InvalidTypeFound { expected, found } => TypeCheckerError::InvalidTypeFound {
                expected: expected.to_owned(),
                found: found.to_owned(),
            },
            Self::InvalidTypesFound { expected, found } => TypeCheckerError::InvalidTypesFound {
                expected: expected.to_owned(),
                found: found.to_owned(),
            },
        }
    }
}

impl TypeCheckerError<'_> {
    pub fn to_short_error(&'_ self) -> ShortTypeCheckerError<'_> {
        match self {
            Self::FunctionNotFound(f) => ShortTypeCheckerError::FunctionNotFound(f.to_string()),
            Self::InvalidTypeFound { expected, found } => ShortTypeCheckerError::InvalidTypeFound {
                expected: expected.to_owned(),
                found: found.to_owned(),
            },
            Self::InvalidTypesFound { expected, found } => {
                ShortTypeCheckerError::InvalidTypesFound {
                    expected: expected.to_owned(),
                    found: found.to_owned(),
                }
            }
        }
    }
}
