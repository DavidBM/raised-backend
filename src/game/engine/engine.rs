use crate::game::structs::{Effect, Intention};
use crate::game::engine::structs::Position;
use crate::game::domain::mutators::apply_effects;
use crate::game::domain::pj::Pj;
use std::sync::RwLock;
use crate::game::structs::PlayerIntention;
use crate::game::domain::world::{WorldUpdate, WorldHistory, World};
use crate::game::systems::{PjMovement, System, PjConnection};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{Sender, channel, Receiver};

#[derive(Debug)]
pub struct Runner {
	version: u64,
	world: Arc<RwLock<WorldHistory>>,
	player_intention_buffer: Vec<PlayerIntention>,
	systems: Vec<Box<dyn System>>,
	systems_senders: Vec<Sender<(Arc<RwLock<WorldHistory>>, u32)>>,
	systems_receivers: Vec<Receiver<Vec<Effect>>>,
}

impl <'a> Runner {
	pub fn new() -> Runner {
		let initial_world = World::new();
		let world_history = Arc::new(RwLock::new(WorldHistory::new(initial_world)));

		let mut runner = Runner {
			version: 0u64, 
			world: world_history, 
			player_intention_buffer: Vec::new(),
			systems: Vec::new(),
			systems_receivers: Vec::new(),
			systems_senders: Vec::new(),
		};

		runner.add_system(Box::new(PjMovement {}));
		runner.add_system(Box::new(PjConnection {}));

		runner
	}

	fn add_system(&mut self, mut system: Box<dyn System>) {
		let (sender, receiver) = channel::<Vec<Effect>>();
		let (go_sender, go_receiver) = channel::<(Arc<RwLock<WorldHistory>>, u32)>();

		self.systems_senders.push(go_sender);
		self.systems_receivers.push(receiver);

		let _ = thread::Builder::new().name("system".to_string()).spawn(move || {
			loop {
				let (world, elapsed) = go_receiver.recv().unwrap();

				let world = world.read().expect("Cannot get world read lock for executing service");

				let effects = system.execute_tick(&world, elapsed);

				drop(world);

				sender.send(effects).unwrap();
			}
		});
	}

	pub fn update(&mut self, elapsed: u32) -> WorldUpdate {

		self.set_player_intentions_in_world();

		let update = self.execute_systems(elapsed);

		self.apply_effects(&mut self.world.write().unwrap(), &update);

		update
	}

	fn set_player_intentions_in_world(&mut self) {
		let world = self.world.write().unwrap().get_current();

		let mut world = world.write().unwrap();

		for intention in &self.player_intention_buffer {
			for player in world.players.iter_mut() {
				if player.id == intention.player_id {
					player.intention.push(intention.intention.clone());
				}
			}
		}

		self.player_intention_buffer.clear();
	}

	fn execute_systems(&mut self, elapsed: u32) -> WorldUpdate {
		let mut updates = WorldUpdate::new();
		
		for sender in self.systems_senders.iter() {
			sender.send((self.world.clone(), elapsed)).unwrap();
		}

		for receivers in self.systems_receivers.iter() {
			let effects = receivers.recv().unwrap();
		
			for effect in effects {
				updates.add_pach(effect);
			}
		}

		updates
	}

	fn apply_effects(&self, world_history: &mut WorldHistory, updates: &WorldUpdate) {
		let mut world = (*world_history.get_current().write().unwrap()).clone();

		apply_effects(&updates.patchs, &mut world);

		world.version += 1;

		world_history.update(world);
	}

	pub fn add_player(&mut self, player_id: u64) {
		let mut world = self.world.write().unwrap();

		let world = world.get_current();

		world.write().unwrap().players.push(Pj {
			id: player_id, 
			position: Position {x: 0.0, y: 0.0, z: 0.0}, 
			intention: vec!(Intention::ConnectPlayer)}
		);
	}

	pub fn set_players_intention(&mut self, intentions: Vec<PlayerIntention>) {
		for intention in intentions {
			self.player_intention_buffer.push(intention);
		}
	}
}
