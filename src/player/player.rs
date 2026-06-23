use crate::player::player_id::PlayerId;
use crate::player::player_screen_name::PlayerScreenName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub screen_name: PlayerScreenName,
}
