use super::terrain::TerrainFeature;
use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell, CELL_SIZE, GRID_SIZE};
use crate::terrain::base::NoiseGenerator;
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
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
		let color = self.get_color();

		commands.spawn(SpriteBundle {
			sprite: Sprite {
				color,
				custom_size: Some(Vec2::new(screen_cell.cell_size, screen_cell.cell_size)),
				..default()
			},
			transform: Transform::from_xyz(
				screen_cell.x as f32 * screen_cell.cell_size,
				screen_cell.y as f32 * screen_cell.cell_size,
				0.0,
			),
			..default()
		});
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
		biome_value: f64,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
	) -> Self {
		if water_type.is_water() {
			return Biome::Desert;
		}

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
		pos: (usize, usize),
		deps: &(Layer<WaterType>, Layer<TerrainFeature>),
	) -> Biome {
		let water_type = deps.0.get(pos.0, pos.1);
		let terrain_feature = deps.1.get(pos.0, pos.1);
		let value = self.noise_gen.get_biome_value(pos.0, pos.1);
		Biome::from_values(value, water_type, terrain_feature)
	}
}

pub fn generate_biome_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
) -> Layer<Biome> {
	let factory = BiomeLayerFactory::new(noise_gen.clone());
	super::generate_layer(
		scale,
		(water_layer, terrain_layer),
		factory,
		super::GridPositionIterator::new(scale),
	)
}
