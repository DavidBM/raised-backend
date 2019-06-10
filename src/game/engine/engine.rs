use crate::game::structs::Effect;
use crate::game::engine::structs::Position;
use crate::game::entities::domain::pj::Pj;
use std::sync::RwLock;
use crate::game::structs::PlayerIntention;
use crate::game::entities::domain::world::{WorldUpdate, WorldHistory, World};
use crate::game::entities::{PjMovement, System};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{Sender, channel, Receiver};

#[derive(Debug)]
pub struct Runner {
	version: u64,
	world: Arc<RwLock<WorldHistory>>,
	player_intention_buffer: Vec<PlayerIntention>,
	systems: Vec<Box<dyn System>>,
	systems_senders: Vec<Sender<Arc<RwLock<WorldHistory>>>>,
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

		runner
	}

	fn add_system(&mut self, mut system: Box<dyn System>) {
		let (sender, receiver) = channel::<Vec<Effect>>();
		let (go_sender, go_receiver) = channel::<Arc<RwLock<WorldHistory>>>();

		self.systems_senders.push(go_sender);
		self.systems_receivers.push(receiver);

		thread::spawn(move || {
			loop {
				let world = go_receiver.recv().unwrap();

				let effects = system.execute_tick(&world.read().unwrap());

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
		let world = self.world.write().unwrap();

		let world = world.get_current();

		let mut world = match world {
			Some(world) => world,
			None => return (),
		};

		for intention in &self.player_intention_buffer {
			for player in world.players.iter_mut() {
				if player.id == intention.player_id {
					player.intention = Some(intention.intention.clone());
				}
			}
		}
	}

	fn execute_systems(&mut self, _elapsed: u32) -> WorldUpdate {
		let mut updates = WorldUpdate::new();
		
		for sender in self.systems_senders.iter() {
			sender.send(self.world.clone()).unwrap();
		}

		for receivers in self.systems_receivers.iter() {
			let effects = receivers.recv().unwrap();
		
			for effect in effects {
				updates.add_pach(effect);
			}
		}

		updates
	}

	fn apply_effects(&self, _world: &mut WorldHistory, updates: &WorldUpdate) {
		for update in updates.patchs.iter() {
			match update {
				Effect::PlayerMoved{..} => (),
				_ => (),
			}
		}
	}

	pub fn add_player(&mut self, player_id: u64) {
		let world = self.world.write().unwrap();

		let world = world.get_current();

		let mut world = match world {
			Some(world) => world,
			None => return (),
		};

		world.players.push(Pj {id: player_id, position: Position {x: 0.0, y: 0.0, z: 0.0}, intention: None});
	}

	pub fn set_players_intention(&mut self, intentions: Vec<PlayerIntention>) {
		for intention in intentions {
			self.player_intention_buffer.push(intention);
		}
	}
}
