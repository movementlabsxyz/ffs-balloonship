use crate::layer::base::NoiseGenerator;
use crate::layer::layers::biome::Biome;
use crate::layer::layers::terrain::TerrainFeature;
use crate::layer::layers::water::WaterType;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
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
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
		let color = self.get_color();

		if *self == TerrainDetail::None {
			return;
		}

		commands.spawn((
			Sprite {
				color,
				custom_size: Some(Vec2::new(
					world_cell.cell_size as f32,
					world_cell.cell_size as f32,
				)),
				..default()
			},
			Transform::from_xyz(world_cell.position.x as f32, world_cell.position.x as f32, 0.0),
		));
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
		detail_value: u32,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
		biome: Biome,
	) -> Self {
		// normalize detail_value to 0-1
		let detail_value = detail_value as f64 / u32::MAX as f64;

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
		pos: WorldPosition,
		deps: &(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>),
	) -> TerrainDetail {
		let water_type = deps.0.get(pos);
		let terrain_feature = deps.1.get(pos);
		let biome = deps.2.get(pos);
		let value = self.noise_gen.get_noise_value(&pos, 0);
		TerrainDetail::from_values(value, water_type, terrain_feature, biome)
	}
}

pub fn generate_detail_layer(
	noise_gen: &NoiseGenerator,
	scale: u32,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
) -> Layer<TerrainDetail> {
	let factory = DetailLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer),
		factory,
		crate::layer::AllGridPositions::new(scale),
	)
}
