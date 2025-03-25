use super::biome::Biome;
use super::terrain::TerrainFeature;
use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell, CELL_SIZE, GRID_SIZE};
use crate::terrain::base::NoiseGenerator;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum TerrainDetail {
	#[default]
	None,
	Rock,
	Sand,
	Mud,
	Snow,
	Ice,
}

impl LayerValue for TerrainDetail {
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
		let color = self.get_color();

		if *self == TerrainDetail::None {
			return;
		}

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
			TerrainDetail::None => Color::NONE,
			TerrainDetail::Rock => Color::srgb(0.5, 0.5, 0.5),
			TerrainDetail::Sand => Color::srgb(0.9, 0.8, 0.3),
			TerrainDetail::Mud => Color::srgb(0.4, 0.3, 0.2),
			TerrainDetail::Snow => Color::srgb(1.0, 1.0, 1.0),
			TerrainDetail::Ice => Color::srgb(0.8, 0.9, 1.0),
		}
	}
}

impl TerrainDetail {
	pub fn from_values(
		detail_value: f64,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
		biome: Biome,
	) -> Self {
		if water_type.is_water() {
			if detail_value > 0.7 {
				Self::Ice
			} else {
				Self::None
			}
		} else {
			match biome {
				Biome::Desert => {
					if detail_value > 0.6 {
						Self::Sand
					} else {
						Self::None
					}
				}
				Biome::Tundra | Biome::Snow => {
					if detail_value > 0.6 {
						Self::Snow
					} else if detail_value > 0.3 {
						Self::Rock
					} else {
						Self::None
					}
				}
				_ => {
					if detail_value > 0.7 {
						Self::Rock
					} else if detail_value > 0.5 {
						Self::Mud
					} else {
						Self::None
					}
				}
			}
		}
	}
}

pub struct DetailLayerFactory {
	noise_gen: NoiseGenerator,
}

impl DetailLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl LayerFactory<TerrainDetail, (Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>)>
	for DetailLayerFactory
{
	fn create_value(
		&self,
		pos: (usize, usize),
		deps: &(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>),
	) -> TerrainDetail {
		let water_type = deps.0.get(pos.0, pos.1);
		let terrain_feature = deps.1.get(pos.0, pos.1);
		let biome = deps.2.get(pos.0, pos.1);
		let value = self.noise_gen.get_detail_value(pos.0, pos.1);
		TerrainDetail::from_values(value, water_type, terrain_feature, biome)
	}
}

pub fn generate_detail_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
) -> Layer<TerrainDetail> {
	let factory = DetailLayerFactory::new(noise_gen.clone());
	super::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer),
		factory,
		super::GridPositionIterator::new(scale),
	)
}
