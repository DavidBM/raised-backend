use game::PlayerIntention;
use game::structs::*;
use game::engine::structs::*;

#[derive(Debug)]
pub struct World {

}

impl World {
	pub fn new() -> World {
		World {}
	}

	pub fn process_player_intention(&self, intention: &PlayerIntention) -> (Option<Vec<PlayerNotification>>, Option<Vec<PlayerEffect>>) {
		match intention {
			&PlayerIntention::Move{..} => {
				(None, None)
			}
			&PlayerIntention::None => (None, None),
		}
	}
}
