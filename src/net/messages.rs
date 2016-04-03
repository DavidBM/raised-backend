#[derive(Deserialize, Debug, Clone)]
pub enum Message {
	LoginMessage (LoginMessage),
	PlayerMove (PlayerMove),
	PlayerDisconnected
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageType {
	pub t: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PlayerMove {
	pub direction: f32,
	pub velocity: f32,

}
#[derive(Deserialize, Debug, Clone)]
pub struct LoginMessage {
	pub token: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PlayerDisconnected;

#[derive(Serialize, Debug, Clone)]
pub enum SendMessage {
	PlayerMove {
		x: f32,
		y: f32,
		z: f32,
		direction: f32,
		id: String
	},
}
