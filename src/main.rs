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

fn main() {
	net::ws_server::start("127.0.0.1:3012");
}
