use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Lobby {
    pub id: i32,
    pub name: String,
    pub state: LobbyState,
    pub max_players: i32,
}

impl Lobby {
    pub fn new(id: Option<i32>, name: Option<String>, state: Option<i32>, max_players: Option<i32>) -> Option<Self> {
        match (id, name, state, max_players) {
            (Some(id), Some(name), Some(state), Some(max_players)) => {
                Some(Lobby {
                    id,
                    name,
                    state: LobbyState::from_i32(state),
                    max_players,
                })
            },
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LobbyState {
    WAITING,
    CONFIGURING,
    ONGOING,
    FINISHED,
}

impl LobbyState {
    pub fn from_i32(state: i32) -> Self {
        match state {
            0 => LobbyState::WAITING,
            1 => LobbyState::CONFIGURING,
            2 => LobbyState::ONGOING,
            _ => LobbyState::FINISHED,
        }
    }
}