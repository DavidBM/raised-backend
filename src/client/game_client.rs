use std::sync::mpsc;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	receiver: mpsc::Receiver<()>
}

impl GameClient {
    pub fn new(id: String, receiver: mpsc::Receiver<()>) -> GameClient {
        GameClient {id: id, receiver: receiver}
    }
}
