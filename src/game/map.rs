use game::PlayerIntention;

#[derive(Debug, Clone)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(Debug, Clone)]
pub enum PlayerNotification {
	Position {
		player_id: u64,
		to: PlayersSelector,
		position: Position,
		direction: f32
	}
}

#[derive(Debug, Clone)]
pub enum PlayersSelector {
	//OnePlayer(u64),
	//SomePlayers(Vec<u64>),
	AllPlayers
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

#[derive(Debug)]
pub struct Map {

}

impl Map {
	pub fn new() -> Map {
		Map {}
	}

	pub fn process_player_intention(&self, intention: &PlayerIntention) -> (Option<Vec<PlayerNotification>>, Option<Vec<PlayerEffect>>) {
		match intention {
			&PlayerIntention::Move{x, y, z, player_id, direction, ..} => {
				let position = Position {x: x, y: y, z: z};
				(
					Some(vec![PlayerNotification::Position {
						player_id: player_id,
						to: PlayersSelector::AllPlayers,
						position: position.clone(),
						direction: direction
					}]),
					Some(vec![PlayerEffect::Position {player_id: player_id, position: position}])
				)
			}
			&PlayerIntention::None => (None, None),
		}
	}
}
