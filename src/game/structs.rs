use crate::game::engine::structs::Position;
use crate::net;
use uuid::Uuid;

#[derive(Debug)]
pub enum ClientActions {
    New(net::GameClient),
    Delete(Uuid),
}

#[derive(Debug, Clone)]
pub struct PlayerIntention {
    pub player_id: u64,
    pub intention: Intention,
}

#[derive(Debug, Clone)]
pub enum Intention {
    Move { direction: f32 },
    ConnectPlayer,
    DisconnectPlayer,
    None,
}

#[derive(Debug, Clone, Serialize)]
pub enum Effect {
    PlayerMoved { player_id: u64, position: Position },
    PlayerConnected(u64),
    PlayerDiconnected(u64),
}

impl Effect {
    pub fn get_player_id(&self) -> Option<u64> {
        match *self {
            Effect::PlayerMoved { player_id, .. } => Some(player_id),
            Effect::PlayerConnected(player_id) => Some(player_id),
            Effect::PlayerDiconnected(player_id) => Some(player_id),
        }
    }
}
