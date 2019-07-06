extern crate casey;
extern crate env_logger;
extern crate paste;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate ws;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod config;
mod game;
mod net;

use crate::game::structs::ClientActions;
use crate::game::WaitingQueue;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("warn,ws=info")).init();
    warn!("Starting server...");

    let (actions_sender, actions_receiver) = channel::<ClientActions>();

    thread::spawn(move || {
        let mut waiting_queue = WaitingQueue::new(actions_receiver);
        trace!("WaitingQueue created");
        waiting_queue.wait_clients()
    });

    net::ws_server::start("127.0.0.1:3012", actions_sender)
}
