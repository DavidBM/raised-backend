pub mod server {
	use ws::{listen, Handler, Result, Message, Handshake, CloseCode, Error};
	use std::sync::mpsc::{Sender, Receiver};
	use std::sync::mpsc;
	use std::thread;


	use client::{WsClient, GameClient};
	use waiting_queue::WaitingQueue;

	impl Handler for WsClient {

		fn on_open(&mut self, _: Handshake) -> Result<()> {
			Ok(println!("New client connected"))
		}

		fn on_message(&mut self, msg: Message) -> Result<()> {
			match msg.into_text() {
				Ok(message) => self.proccess_message(message),
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
		}

		fn on_error(&mut self, err: Error) {
			println!("The server encountered an error: {:?}", err);
		}
	}


	pub fn start() {
		let (tx_wq, rx_wq): (Sender<GameClient>, Receiver<GameClient>) = mpsc::channel();

		thread::spawn(move || {
			let mut waiting_queue = WaitingQueue::new(rx_wq);
			waiting_queue.wait_clients();
		});

		listen("127.0.0.1:3012", |out| {
			let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

			let ws_client = WsClient::new(out, tx);

			let id = ws_client.get_id();

			let client = GameClient::new(id, rx);

			tx_wq.send(client).unwrap();

			ws_client
		}).unwrap()
	}

}
