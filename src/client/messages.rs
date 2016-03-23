pub trait Message {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginMessage {
	pub token: String,
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

impl Message for LoginMessage {}
impl Message for MessageType {}
impl Message for PlayerMove {}
