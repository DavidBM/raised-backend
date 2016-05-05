#[derive(Debug, Clone)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(Debug, Clone)]
pub struct WorldPatch;

//TODO: Improve this with indexes for getting faster the patchs of one type
#[derive(Debug)]
pub struct WorldUpdate {
	pub version: u64,
	pub patchs: Vec<WorldPatch>
}

impl WorldUpdate {
	pub fn new (version: u64) -> WorldUpdate {
		WorldUpdate {version: version, patchs: Vec::new()}
	}

	pub fn add_pach(& mut self, patch: WorldPatch) {
		let index = self.patchs.len();
		self.patchs.push(patch);
	}

	pub fn get_player_patchs(&self) -> Vec<WorldPatch> {
		let patches = self.patchs.iter().filter_map(|patch| {
			Some(patch)
		}).cloned().collect();

		patches
	}
}
