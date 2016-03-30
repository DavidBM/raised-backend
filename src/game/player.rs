use net::GameClient as Client;
use net::Message as Actions;

#[derive(Debug)]
pub struct Player {
	net: Client
}

impl Player {
	pub fn new(client: Client) -> Player {
		Player {net: client}
	}

	pub fn get_actions(&self) -> Option<Vec<Actions>> {
		self.net.get_messages()
	}

	pub fn process_message(&self, message: Actions, elapsed: u64) -> Result<(), ()> {
		match message {
			Actions::PlayerMove(_) => Ok(()),
			Actions::LoginMessage(_) => Ok(()),
			Actions::PlayerDisconnected => Ok(()),
		}
	}
}
