use crate::game::engine::structs::Position;
use crate::game::Intention;

#[derive(Debug, Clone)]
pub struct Pj {
	pub id: u64,
	pub position: Position,
	pub intention: Vec<Intention>
}
