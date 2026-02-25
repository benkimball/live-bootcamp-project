use std::str::FromStr;

use crate::utils::auth::TwoFACodeError;

#[derive(Debug, Clone, PartialEq)]
pub struct TwoFACode(String);

impl FromStr for TwoFACode {
    type Err = TwoFACodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 6 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(s.to_string()))
        } else {
            Err(TwoFACodeError::Invalid)
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        Self(rand::random_range(100_000..999_999).to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_code_must_be_six_digits(code: String) -> bool {
        code.len() == 6 || TwoFACode::from_str(&code).is_err()
    }

    #[quickcheck]
    fn prop_code_must_be_numeric(code: String) -> bool {
        code.chars().all(|c| c.is_ascii_digit()) || TwoFACode::from_str(&code).is_err()
    }

    #[quickcheck]
    fn prop_default_must_be_valid() -> bool {
        TwoFACode::default().as_ref().parse::<TwoFACode>().is_ok()
    }
}
