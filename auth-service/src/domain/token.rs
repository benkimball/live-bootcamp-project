use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

/// Represents a token used for authentication. May be
/// valid or invalid; the type makes no guarantees about
/// the content or validity of the token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token(String);

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token(value.to_string())
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token(value)
    }
}

impl From<Token> for Cow<'_, str> {
    fn from(value: Token) -> Self {
        Self::Owned(value.0)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
