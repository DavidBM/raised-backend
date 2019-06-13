#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
	PlayerMove {
		x: f32,
		y: f32,
		z: f32,
		direction: f32,
		id: String
	},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PacketType {
	pub t: String,
}

#[derive(Debug, Clone)]
pub enum ClientPacket {
	Disconnected,
	Login(Login),
	Move(Move),
	Stay,
	Attack(Attack),
	Equip(Equip)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
	token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Move {
    pub direction: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attack {
    direction: f32,
    slot: Slot
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equip {
    item: Item,
    slot: Slot
}

/*Helper structs*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slot {
    id: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: u32
}
