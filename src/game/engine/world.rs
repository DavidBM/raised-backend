use game::structs::*;
use game::engine::structs::*;

#[derive(Debug)]
pub struct World {

}

impl World {
	pub fn new() -> World {
		World {}
	}

	pub fn update(&self, elapsed: u32) -> WorldUpdate {
		//bla bla bla a lot of code
		WorldUpdate::new(1u64)
	}
}
