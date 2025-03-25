use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell, CELL_SIZE, GRID_SIZE};
use crate::terrain::base::NoiseGenerator;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum TerrainFeature {
	#[default]
	Plains,
	Mountain,
	Valley,
	Canyon,
	Cliff,
}

impl LayerValue for TerrainFeature {
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
			TerrainFeature::Plains => Color::srgb(0.5, 0.5, 0.5),
			TerrainFeature::Mountain => Color::srgb(0.7, 0.7, 0.7),
			TerrainFeature::Valley => Color::srgb(0.3, 0.3, 0.3),
			TerrainFeature::Canyon => Color::srgb(0.2, 0.2, 0.2),
			TerrainFeature::Cliff => Color::srgb(0.6, 0.6, 0.6),
		}
	}
}

impl TerrainFeature {
	pub fn from_values(terrain_value: f64, water_type: WaterType) -> Self {
		if water_type.is_water() {
			return Self::Plains;
		}

		if terrain_value > 0.8 {
			Self::Mountain
		} else if terrain_value > 0.6 {
			Self::Cliff
		} else if terrain_value > 0.4 {
			Self::Plains
		} else if terrain_value > 0.2 {
			Self::Valley
		} else {
			Self::Canyon
		}
	}
}

pub struct TerrainLayerFactory {
	noise_gen: NoiseGenerator,
}

impl TerrainLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl LayerFactory<TerrainFeature, Layer<WaterType>> for TerrainLayerFactory {
	fn create_value(&self, pos: (usize, usize), water_layer: &Layer<WaterType>) -> TerrainFeature {
		let water_type = water_layer.get(pos.0, pos.1);
		let value = self.noise_gen.get_terrain_value(pos.0, pos.1);
		TerrainFeature::from_values(value, water_type)
	}
}

pub fn generate_terrain_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
) -> Layer<TerrainFeature> {
	let factory = TerrainLayerFactory::new(noise_gen.clone());
	super::generate_layer(scale, water_layer, factory, super::GridPositionIterator::new(scale))
}
