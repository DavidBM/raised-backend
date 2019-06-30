use crate::game::domain::mutators::apply_effects;
use crate::game::domain::pj::Pj;
use crate::game::domain::world::{World, WorldHistory, WorldUpdate};
use crate::game::engine::structs::Position;
use crate::game::structs::PlayerIntention;
use crate::game::structs::{Effect, Intention};
use crate::game::systems::{PjConnection, PjMovement, System};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

#[derive(Debug)]
pub struct Runner {
    version: u64,
    world: Arc<RwLock<WorldHistory>>,
    player_intention_buffer: Vec<PlayerIntention>,
    systems: Vec<Box<dyn System>>,
    systems_senders: Vec<Sender<(Arc<RwLock<WorldHistory>>, u32)>>,
    systems_receivers: Vec<Receiver<Vec<Effect>>>,
    threads_handler: Vec<std::thread::JoinHandle<()>>,
}

impl<'a> Runner {
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
            threads_handler: Vec::new(),
        };

        runner.add_system(Box::new(PjMovement {}));
        runner.add_system(Box::new(PjConnection {}));

        runner
    }

    fn add_system(&mut self, mut system: Box<dyn System>) {
        let (effects_sender, effects_receiver) = channel::<Vec<Effect>>();
        let (tick_sender, tick_receiver) = channel::<(Arc<RwLock<WorldHistory>>, u32)>();

        self.systems_senders.push(tick_sender);
        self.systems_receivers.push(effects_receiver);

        let thread_error_message = format!("Failed to create thread for System {:?}", system);

        let thread_handler = thread::Builder::new()
            .name(format!("{} System", system))
            .spawn(move || loop {
                let (world, elapsed) = tick_receiver.recv().unwrap();

                let world = world.read().expect("Cannot get world read lock for executing service");

                let effects = system.execute_tick(&world, elapsed);

                drop(world);

                effects_sender.send(effects).unwrap();
            })
            .expect(&thread_error_message);

        self.threads_handler.push(thread_handler);
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
            if let Ok(effects) = receivers.recv() {
                updates.add_pach(effects);
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
            position: Position { x: 0.0, y: 0.0, z: 0.0 },
            intention: vec![Intention::ConnectPlayer],
        });
    }

    pub fn set_players_intention(&mut self, intentions: Vec<PlayerIntention>) {
        for intention in intentions {
            self.player_intention_buffer.push(intention);
        }
    }
}
