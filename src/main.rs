#![feature(proc_macro)]
#![feature(plugin)]

extern crate ws;
extern crate uuid;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate time;
#[macro_use]
extern crate mac;
#[macro_use]
extern crate serde_derive;

mod game;
mod net;
mod config;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel as Channel};
use game::WaitingQueue;
use game::structs::ClientActions;

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









/*
use std::rc::Rc;

#[derive(Debug)]
struct Color {
	red: u8,
	green: u8,
	blue: u8
}

#[derive(Debug)]
struct Test {
	field: Vec<Rc<Color>>,
	red_ones: Vec<Rc<Color>>,
	blue_ones: Vec<Rc<Color>>,
	green_ones: Vec<Rc<Color>>,
}


fn main() {
	//test();

	let mut test = Test {field: Vec::new(), red_ones: Vec::new(), blue_ones: Vec::new(), green_ones: Vec::new()};

	let color = Color {red: 255u8, green: 255u8, blue: 255u8};
	let color = Rc::new(color);

	test.field.push(color.clone());
	test.red_ones.push(color);
}

*/
/*
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;

fn main() {
    test();
}

#[derive(Debug)]
struct Bla {
	pub field: u64,
	pub hola: Vec<f32>,
	pub bla: Ble
}

#[derive(Debug)]
struct Ble {
	field: u32
}

fn test() {
	let bla = Bla {field: 45u64, hola: Vec::new(), bla: Ble {field: 199u32}};
	let structure = RwLock::new(bla);
	let arc = Arc::new(structure);

	let reference = arc.clone();
	let child1 = thread::spawn(move || {
		println!("{:?}", reference.read().unwrap());
	});

	let reference = arc.clone();
	let child2 = thread::spawn(move || {
		println!("{:?}", reference.read().unwrap());
		//Now I can send safely a signal to the main thread to update the world
	});

	child1.join();
	child2.join();

	{
		arc.write().unwrap().field = 98u64;
	}

	let reference = arc.clone();
	let child1 = thread::spawn(move || {
		println!("{:?}", reference.read().unwrap());
	});

	let reference = arc.clone();
	let child2 = thread::spawn(move || {
		println!("{:?}", reference.read().unwrap());
	});

	child1.join();
	child2.join();
}
*/
