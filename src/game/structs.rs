use crate::net;
use uuid::Uuid;
use crate::game::engine::structs::Position;

#[derive(Debug)]
pub enum ClientActions {
	New(net::GameClient),
	Delete(Uuid)
}

#[derive(Debug, Clone)]
pub enum PlayerEffect {
	Position {
		player_id: u64,
		position: Position
	}
}

impl PlayerEffect {
	pub fn get_id(&self) -> Option<u64> {
		match self {
			&PlayerEffect::Position{player_id, ..} => Some(player_id),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Intention {
	Move {
		player_id: u64,
		direction: f32,
	},
	None
}
