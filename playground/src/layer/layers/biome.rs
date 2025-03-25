use crate::layer::base::NoiseGenerator;
use crate::layer::layers::terrain::TerrainFeature;
use crate::layer::layers::water::WaterType;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Biome {
	#[default]
	Desert,
	Grassland,
	Forest,
	Jungle,
	Tundra,
	Snow,
}

impl LayerValue for Biome {
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
		let color = self.get_color();

		commands.spawn((
			Sprite {
				color,
				custom_size: Some(Vec2::new(
					world_cell.cell_size as f32,
					world_cell.cell_size as f32,
				)),
				..default()
			},
			Transform::from_xyz(world_cell.position.x as f32, world_cell.position.y as f32, 0.0),
		));
	}

	fn get_color(&self) -> Color {
		match self {
			Biome::Desert => Color::srgb(0.9, 0.8, 0.3),
			Biome::Grassland => Color::srgb(0.4, 0.8, 0.2),
			Biome::Forest => Color::srgb(0.2, 0.6, 0.1),
			Biome::Jungle => Color::srgb(0.1, 0.5, 0.1),
			Biome::Tundra => Color::srgb(0.8, 0.8, 0.8),
			Biome::Snow => Color::srgb(1.0, 1.0, 1.0),
		}
	}
}

impl Biome {
	pub fn from_values(
		biome_value: u32,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
	) -> Self {
		if water_type.is_water() {
			return Biome::Desert;
		}

		// normalize biome_value to 0-1
		let biome_value = biome_value as f64 / u32::MAX as f64;

		match terrain_feature {
			TerrainFeature::Mountain => {
				if biome_value > 0.5 {
					Self::Snow
				} else {
					Self::Tundra
				}
			}
			TerrainFeature::Plains => {
				if biome_value > 0.7 {
					Self::Desert
				} else if biome_value > 0.4 {
					Self::Grassland
				} else {
					Self::Forest
				}
			}
			_ => {
				if biome_value > 0.6 {
					Self::Jungle
				} else if biome_value > 0.3 {
					Self::Forest
				} else {
					Self::Grassland
				}
			}
		}
	}
}

pub struct BiomeLayerFactory {
	noise_gen: NoiseGenerator,
}

impl BiomeLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl LayerFactory<Biome, (Layer<WaterType>, Layer<TerrainFeature>)> for BiomeLayerFactory {
	fn create_value(
		&self,
		pos: WorldPosition,
		deps: &(Layer<WaterType>, Layer<TerrainFeature>),
	) -> Biome {
		let water_type = deps.0.get(pos);
		let terrain_feature = deps.1.get(pos);
		let value = self.noise_gen.get_noise_value(&pos, 0);
		Biome::from_values(value, water_type, terrain_feature)
	}
}

pub fn generate_biome_layer(
	noise_gen: &NoiseGenerator,
	scale: u32,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
) -> Layer<Biome> {
	let factory = BiomeLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(
		scale,
		(water_layer, terrain_layer),
		factory,
		crate::layer::AllGridPositions::new(scale),
	)
}
