use std::borrow::Cow;

use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
