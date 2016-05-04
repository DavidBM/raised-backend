use game::engine::structs::Position;
use config::player::speed;
use std::f32::consts::PI as PIf32;

#[derive(Debug)]
struct Pj {
	position: Position
}

impl Pj {
	fn new() -> Pj {
		Pj{position: Position{x: 0f32, y: 0f32, z: 0f32}}
	}

	fn move_direction(&self, direction: f32, elapsed: u32) {
		let x = self.position.x + direction.cos() * speed as f32 * (elapsed as f32 / 1_000_000_f32);
		let y = self.position.y + direction.sin() * speed as f32 * (elapsed as f32 / 1_000_000_f32);
		let mut direction = f32::atan2(y, x);

		if direction < 0_f32 {
			direction += 2_f32 * PIf32;
		}
	}
}
