use serde::Serialize;
use serde_with::DeserializeFromStr;
use std::str::FromStr;

#[derive(Debug, derive_more::Display, Clone, PartialEq, Serialize, DeserializeFromStr)]
pub struct PlayerScreenName(String);

impl PlayerScreenName {
    const MAX_LEN: usize = 16;
}

impl FromStr for PlayerScreenName {
    type Err = InvalidPlayerScreenName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            Err(InvalidPlayerScreenName::Empty)?
        }
        if trimmed.chars().count() > Self::MAX_LEN {
            Err(InvalidPlayerScreenName::Long)?
        }

        Ok(Self(trimmed.into()))
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum InvalidPlayerScreenName {
    #[error("Player screen exceeds the maximum length: {}", PlayerScreenName::MAX_LEN)]
    Long,
    #[error("Player screen name is empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn rejects_empty_strings() {
        let empty = String::from("");
        assert_eq!(PlayerScreenName::from_str(&empty), Err(InvalidPlayerScreenName::Empty));
        assert!(serde_json::from_value::<PlayerScreenName>(Value::String(empty)).is_err());

        let spaces = String::from("  ");
        assert_eq!(PlayerScreenName::from_str(&spaces), Err(InvalidPlayerScreenName::Empty));
        assert!(serde_json::from_value::<PlayerScreenName>(Value::String(spaces)).is_err());
    }

    #[test]
    fn rejects_strings_over_max_len() {
        let over_max = "a".repeat(PlayerScreenName::MAX_LEN + 1);
        assert_eq!(
            PlayerScreenName::from_str(&over_max),
            Err(InvalidPlayerScreenName::Long)
        );
        assert!(serde_json::from_value::<PlayerScreenName>(Value::String(over_max)).is_err());
    }

    #[test]
    fn accepts_valid_inputs() {
        let min = String::from("a");
        assert_eq!(PlayerScreenName::from_str(&min), Ok(PlayerScreenName(min.clone())));
        assert_eq!(
            serde_json::from_value::<PlayerScreenName>(Value::String(min.clone())).unwrap(),
            PlayerScreenName(min)
        );

        let max = "a".repeat(PlayerScreenName::MAX_LEN);
        assert_eq!(PlayerScreenName::from_str(&max), Ok(PlayerScreenName(max.clone())));
        assert_eq!(
            serde_json::from_value::<PlayerScreenName>(Value::String(max.clone())).unwrap(),
            PlayerScreenName(max)
        );

        let not_trimmed = String::from("  abc     ");
        let trimmed = String::from("abc");
        assert_eq!(
            PlayerScreenName::from_str(&not_trimmed),
            Ok(PlayerScreenName(trimmed.clone()))
        );
        assert_eq!(
            serde_json::from_value::<PlayerScreenName>(Value::String(not_trimmed)).unwrap(),
            PlayerScreenName(trimmed)
        );
    }
}
