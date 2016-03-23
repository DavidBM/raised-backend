use std::sync::mpsc;
use std::{thread, time};
use client::Message as GameMessage;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	receiver: mpsc::Receiver<GameMessage>
}

impl GameClient {
	pub fn new(id: String, receiver: mpsc::Receiver<GameMessage>) -> GameClient {
		GameClient {id: id, receiver: receiver}
	}

	pub fn get_message(&self) -> Vec<GameMessage> {
		let mut messages: Vec<GameMessage> = Vec::new();

		while let Ok(game_message) = self.receiver.try_recv() {
			messages.push(game_message);
		}

		println!("{:?}s received", messages.len());

		thread::sleep(time::Duration::new(1, 0));

		messages
	}
}
