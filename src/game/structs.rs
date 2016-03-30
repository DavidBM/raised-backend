use net;

#[derive(Debug)]
pub enum ClientActions {
	New(net::GameClient),
	Delete(String)
}
