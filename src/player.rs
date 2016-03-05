extern crate uuid;

#[derive(Debug)]
pub struct Player {
	id: String,
	validated: bool,
}

impl Player {
	pub fn new() -> Player {
		Player {
			id: uuid::Uuid::new_v4().to_simple_string(),
			validated: false
		}
	}

	pub fn proccess_message(&self, message: String) -> () {
		println!("New message: '{}'", message)
	}
}
