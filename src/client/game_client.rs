use std::sync::mpsc;

use std::{thread, time};
use std::fmt;
use std::fmt::Debug;
use client::Message;

#[derive(Debug)]
pub struct GameClient {
	id: String,
	receiver: mpsc::Receiver<Box<Message + Send>>
}

impl GameClient {
	pub fn new(id: String, receiver: mpsc::Receiver<Box<Message + Send>>) -> GameClient {
		GameClient {id: id, receiver: receiver}
	}

	pub fn get_message(&self) {
		let mut messages: Vec<Box<Message>> = Vec::new();

		loop {
			let message = self.receiver.try_recv();
			match message {
				Ok(game_message) => {
					messages.push(game_message);
				},
				Err(_) => break,
			}
		}
		thread::sleep(time::Duration::new(1, 0));
		println!("{:?}s received", messages.len());
	}
}

impl Debug for Box<Message> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "I'm a structure :D",)
	}
}
