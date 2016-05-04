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
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
