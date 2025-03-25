use crate::layer::WorldPosition;
use rand::prelude::*;

#[derive(Clone)]
pub struct NoiseGenerator {
	seed: u32,
	rng: ThreadRng,
}

impl NoiseGenerator {
	pub fn new(seed: u32) -> Self {
		Self { seed, rng: rand::thread_rng() }
	}

	// Simple deterministic noise function based on coordinates and layer
	fn get_noise_value(&self, pos: &WorldPosition, salt: u32) -> u32 {
		let combined = (pos.x as u32) ^ (pos.y as u32) ^ (salt as u32) ^ self.seed;
		let mut rng = rand::rngs::StdRng::seed_from_u32(combined);
		rng.gen()
	}
}
