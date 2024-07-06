use std::time::SystemTime;

use crate::players::Players;
use path_serde::{PathIntrospect, PathSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PathSerialize, PathIntrospect, PartialEq)]

pub struct LastFilteredGameControllerStateChanges {
    pub game_state: SystemTime,
    pub opponent_game_state: SystemTime,
    pub game_phase: SystemTime,
    pub kicking_team: SystemTime,
    pub penalties: Players<Option<SystemTime>>,
    pub sub_state: Option<SystemTime>,
}

impl Default for LastFilteredGameControllerStateChanges {
    fn default() -> Self {
        Self {
            game_state: SystemTime::now(),
            opponent_game_state: SystemTime::now(),
            game_phase: SystemTime::now(),
            kicking_team: SystemTime::now(),
            penalties: Players {
                one: None,
                two: None,
                three: None,
                four: None,
                five: None,
                six: None,
                seven: None,
            },
            sub_state: None,
        }
    }
}
