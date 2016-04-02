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
		player_id: u64,
		x: f32,
		y: f32,
		z: f32,
	}
}

impl PlayerEffects {
    pub fn get_id(&self) -> Option<u64> {
		match self {
    	    &PlayerEffects::Position{player_id, ..} => Some(player_id),
    	}
    }
}

#[derive(Debug)]
pub struct Map {

}

impl Map {
	pub fn new() -> Map {
		Map {}
	}

	pub fn process_player_intention(&self, intention: &PlayerIntention) -> Option<Vec<PlayerEffects>> {
		match intention {
			&PlayerIntention::Move{x, y, z, player_id, ..} => {
				Some(vec![PlayerEffects::Position {x: x, y: y, z: z, player_id: player_id}])
			}
			&PlayerIntention::None => None,
		}
	}
}
