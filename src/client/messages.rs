#[derive(Serialize, Deserialize, Debug)]
pub struct LoginMessage {
	pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageType {
	pub t: String,
}
