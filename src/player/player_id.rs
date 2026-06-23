use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, derive_more::Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

impl PlayerId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        let (seconds, subsec_nanos) = self
            .0
            .get_timestamp()
            .expect("PlayerId based on UUID v7 must have timestamp")
            .to_unix();

        DateTime::UNIX_EPOCH + Duration::seconds(seconds as i64) + Duration::nanoseconds(subsec_nanos as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_different_ids() {
        let player_id = PlayerId::generate();
        let player_id2 = PlayerId::generate();

        assert_ne!(player_id, player_id2);
    }

    #[test]
    fn timestamp_succeeds() {
        PlayerId::generate().timestamp();
    }
}
