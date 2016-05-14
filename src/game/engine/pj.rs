use game::engine::structs::Position;
use config::player::SPEED;
use game::Intention;

#[derive(Debug)]
pub struct Pj {
	pub id: u64,
	position: Position,
	intention: Option<Intention>
}

impl Pj {
	pub fn new(id: u64) -> Pj {
		Pj{position: Position{x: 0f32, y: 0f32, z: 0f32}, id: id, intention: None}
	}

	fn get_next_position(&mut self, direction: f32, elapsed: u32) -> Position {
		let x = self.position.x + direction.cos() * SPEED as f32 * (elapsed as f32 / 1_000_000_f32);
		let y = self.position.y + direction.sin() * SPEED as f32 * (elapsed as f32 / 1_000_000_f32);

		Position{x: x, y: y, z: self.position.z}
	}

	pub fn set_players_intention(&mut self, intention: Intention) {
	    self.intention = Some(intention);
	}
}
