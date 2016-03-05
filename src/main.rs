extern crate ws;
extern crate env_logger;
extern crate rustc_serialize;

use std::rc::Rc;
use std::cell::Cell;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

mod player;

struct Server {
	out: Sender,
	count: Rc<Cell<u32>>,
	player: player::Player
}

impl Handler for Server {

	fn on_open(&mut self, _: Handshake) -> Result<()> {
		println!("New client connected");
		// We have a new connection, so we increment the connection counter

		Ok(self.count.set(self.count.get() + 1))

	}

	fn on_message(&mut self, msg: Message) -> Result<()> {
		match msg.into_text() {
			Ok(message) => self.player.proccess_message(message),
			Err(message) => println!("No message or binnary message: '{}'", message),
		}

		Ok(())
	}

	fn on_close(&mut self, code: CloseCode, reason: &str) {
		match code {
			CloseCode::Normal => println!("The client is done with the connection."),
			CloseCode::Away   => println!("The client is leaving the site."),
			CloseCode::Abnormal => println!("Closing handshake failed! Unable to obtain closing status from client."),
			_ => println!("The client encountered an error: {}", reason),
		}

		// The connection is going down, so we need to decrement the count
		self.count.set(self.count.get() - 1)
	}

	fn on_error(&mut self, err: Error) {
		println!("The server encountered an error: {:?}", err);
	}

}

// Cell gives us interior mutability so we can increment
// or decrement the count between handlers.
// Rc is a reference-counted box for sharing the count between handlers
// since each handler needs to own its contents.
fn main() {
	let count = Rc::new(Cell::new(0));
	listen("127.0.0.1:3012", |out| { Server { out: out, count: count.clone(), player: player::Player::new()} }).unwrap()
}
