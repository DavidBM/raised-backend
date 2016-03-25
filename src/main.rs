#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]

extern crate ws;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

mod waiting_queue;
mod client;
mod ws_server;

use ws_server::server;

fn main() {
	server::start("127.0.0.1:3012");
}
