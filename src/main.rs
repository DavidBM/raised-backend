extern crate ws;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod game;
mod net;
mod config;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel as Channel};
use crate::game::WaitingQueue;
use crate::game::structs::ClientActions;

fn main() {
	env_logger::from_env(env_logger::Env::default().default_filter_or("warn,ws=info")).init();
	warn!("Starting server...");

	let (actions_sender, actions_receiver): (Sender<ClientActions>, Receiver<ClientActions>) = Channel();

	thread::spawn(move || {
		let mut waiting_queue = WaitingQueue::new(actions_receiver);
		trace!("WaitingQueue created");
		waiting_queue.wait_clients()
	});

	net::ws_server::start("127.0.0.1:3012", actions_sender)
}
