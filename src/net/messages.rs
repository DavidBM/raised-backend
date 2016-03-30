#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
	LoginMessage (LoginMessage),
	PlayerMove (PlayerMove),
	PlayerDisconnected
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageType {
	pub t: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerMove {
	pub x: i32,
	pub y: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginMessage {
	pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerDisconnected;
