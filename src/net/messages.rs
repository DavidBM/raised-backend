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
	pub direction: f32,
	pub velocity: f32,

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginMessage {
	pub token: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerDisconnected;
