use thiserror::Error;

use super::types::InternalType;

#[derive(Error, Debug)]
pub enum TypeCheckerError {
    #[error("Function `{0}` does not exist")]
    FunctionNotFound(String),
    #[error("Expected type {expected}, but found {found}")]
    InvalidTypeFound {
        expected: InternalType,
        found: InternalType,
    },
    #[error("Expected types {expected:?}, but found {found:?}")]
    InvalidTypesFound {
        expected: Vec<InternalType>,
        found: Vec<InternalType>,
    },
}

#[derive(Error, Debug)]
pub enum ShortTypeCheckerError {
    #[error("Function `{0}` not found")]
    FunctionNotFound(String),
    #[error("Invalid Type `{found}` Found")]
    InvalidTypeFound {
        expected: InternalType,
        found: InternalType,
    },
    #[error("Invalid Types Found")]
    InvalidTypesFound {
        expected: Vec<InternalType>,
        found: Vec<InternalType>,
    },
}

impl ShortTypeCheckerError {
    pub fn to_long_error(&'_ self) -> TypeCheckerError {
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

impl TypeCheckerError {
    pub fn to_short_error(&'_ self) -> ShortTypeCheckerError {
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
