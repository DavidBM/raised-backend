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

#[derive(Deserialize, Debug, Clone)]
pub struct PacketType {
	pub t: String,
}

pub enum ClientPacket {
	Disconnected,
	Login(Login),
	Move(Move),
	Stay,
	Attack(Attack),
	Equip(Equip)
}

#[derive(Deserialize, Debug, Clone)]
pub struct Login {
	token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Move {
    pub direction: f32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Attack {
    direction: f32,
    slot: Slot
}

#[derive(Deserialize, Debug, Clone)]
pub struct Equip {
    item: Item,
    slot: Slot
}

/*Helper structs*/

#[derive(Deserialize, Debug, Clone)]
pub struct Slot {
    id: u32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    id: u32
}