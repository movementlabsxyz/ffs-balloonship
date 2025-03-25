use rand::prelude::*;

#[derive(Clone)]
pub struct NoiseGenerator {
	seed: u64,
	rng: ThreadRng,
}

impl NoiseGenerator {
	pub fn new(seed: u64) -> Self {
		Self { seed, rng: rand::thread_rng() }
	}

	pub fn get_water_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 0)
	}

	pub fn get_terrain_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 1)
	}

	pub fn get_biome_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 2)
	}

	pub fn get_detail_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 3)
	}

	pub fn get_flora_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 4)
	}

	pub fn get_urban_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 5)
	}

	pub fn get_special_value(&self, x: usize, y: usize) -> f64 {
		self.get_noise_value(x, y, 6)
	}

	// Simple deterministic noise function based on coordinates and layer
	fn get_noise_value(&self, x: usize, y: usize, layer: u8) -> f64 {
		let combined = (x as u64) ^ (y as u64) ^ (layer as u64) ^ self.seed;
		let mut rng = rand::rngs::StdRng::seed_from_u64(combined);
		rng.gen()
	}
}
