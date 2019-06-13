extern crate ws;
extern crate uuid;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate time;
#[macro_use]
extern crate serde_derive;

mod game;
mod net;
mod config;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel as Channel};
use crate::game::WaitingQueue;
use crate::game::structs::ClientActions;

fn main() {
	println!("Starting server...");

	let (channel_sender, channel_receiver): (Sender<ClientActions>, Receiver<ClientActions>) = Channel();

	thread::spawn(move || {
		let mut waiting_queue = WaitingQueue::new(channel_receiver);
		println!("WaitingQueue created");
		waiting_queue.wait_clients()
	});

	net::ws_server::start("127.0.0.1:3012", channel_sender)
}
