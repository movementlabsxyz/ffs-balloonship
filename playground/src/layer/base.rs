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

	/// Simple deterministic noise function based on coordinates and layer
	pub fn get_noise_value(&self, pos: &WorldPosition, salt: u32) -> u32 {
		let combined = (pos.x as u64) ^ (pos.y as u64) ^ (salt as u64) ^ self.seed as u64;
		let mut rng = rand::rngs::StdRng::seed_from_u64(combined);
		rng.gen()
	}
}
