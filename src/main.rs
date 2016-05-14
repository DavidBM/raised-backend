#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]

extern crate ws;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate time;
#[macro_use]
extern crate mac;

mod game;
mod net;
mod config;

/*use std::rc::Rc;

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
*/


fn main() {
	net::ws_server::start("127.0.0.1:3012");
	//test();

/*	let mut test = Test {field: Vec::new(), red_ones: Vec::new(), blue_ones: Vec::new(), green_ones: Vec::new()};

	let color = Color {red: 255u8, green: 255u8, blue: 255u8};
	let color = Rc::new(color);

	test.field.push(color.clone());
	test.red_ones.push(color);*/
}




use std::thread;
use std::sync::Arc;

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
	let vector: Vec<f32> = Vec::new();
	let bla = Bla {field: 45u64, hola: vector, bla: Ble {field: 199u32}};
	let mut arc = Arc::new(bla);

	let reference = arc.clone();
	let child1 = thread::spawn(move || {
		println!("{:p}", reference);
	});

	let reference = arc.clone();
	let child2 = thread::spawn(move || {
		println!("{:p}", reference);
		let weak_ref = Arc::downgrade(&reference);
		drop(reference);
		//Now I can send safely a signal to the main thread to update the world
	});

	child1.join();
	child2.join();

	{
		let mut reft = Arc::get_mut(&mut arc);
		reft.unwrap().field = 98u64
	}

	let reference = arc.clone();
	let child1 = thread::spawn(move || {
		println!("{:p}", reference);
	});

	let reference = arc.clone();
	let child2 = thread::spawn(move || {
		println!("{:p}", reference);
	});

	child1.join();
	child2.join();
}

