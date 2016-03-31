use game::PlayerIntention;

#[derive(Debug)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(Debug, Clone)]
pub enum PlayerEffects {
	Position {
		x: f32,
		y: f32,
		z: f32,
	},
	None
}

#[derive(Debug)]
pub struct Map {

}

impl Map {
	pub fn new() -> Map {
		Map {}
	}

	pub fn process_player_intention(intention: PlayerIntention) -> PlayerEffects {
		match intention {
		    PlayerIntention::Move{x, y, z, ..} => PlayerEffects::Position {x: x, y: y, z: z},
		    PlayerIntention::None => PlayerEffects::None,
		}
	}
}
