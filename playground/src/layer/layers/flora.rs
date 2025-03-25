use crate::layer::base::NoiseGenerator;
use crate::layer::layers::biome::Biome;
use crate::layer::layers::detail::TerrainDetail;
use crate::layer::layers::terrain::TerrainFeature;
use crate::layer::layers::water::WaterType;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Flora {
	#[default]
	None,
	Tree,
	Palm,
	Cactus,
	Bush,
	Flower,
	Mushroom,
	Seaweed,
}

impl LayerValue for Flora {
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
		let color = match self {
			Flora::None => return,
			Flora::Tree => Color::rgb(0.2, 0.5, 0.1),
			Flora::Palm => Color::rgb(0.3, 0.6, 0.2),
			Flora::Cactus => Color::rgb(0.2, 0.4, 0.1),
			Flora::Bush => Color::rgb(0.3, 0.5, 0.2),
			Flora::Flower => Color::rgb(0.8, 0.2, 0.2),
			Flora::Mushroom => Color::rgb(0.7, 0.7, 0.7),
			Flora::Seaweed => Color::rgb(0.1, 0.4, 0.2),
		};

		commands.spawn((
			Sprite {
				color,
				custom_size: Some(Vec2::new(
					world_cell.cell_size as f32,
					world_cell.cell_size as f32,
				)),
				..default()
			},
			Transform::from_xyz(
				world_cell.position.x as f32 * world_cell.cell_size as f32,
				world_cell.position.y as f32 * world_cell.cell_size as f32,
				0.0,
			),
		));
	}

	fn get_color(&self) -> Color {
		match self {
			Self::None => Color::NONE,
			Self::Tree => Color::srgb(0.2, 0.5, 0.2),
			Self::Palm => Color::srgb(0.3, 0.6, 0.3),
			Self::Cactus => Color::srgb(0.3, 0.7, 0.3),
			Self::Bush => Color::srgb(0.3, 0.6, 0.3),
			Self::Flower => Color::srgb(0.8, 0.4, 0.8),
			Self::Mushroom => Color::srgb(0.7, 0.7, 0.7),
			Self::Seaweed => Color::srgb(0.2, 0.4, 0.2),
		}
	}
}

impl Flora {
	pub fn from_values(
		flora_value: f64,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
		biome: Biome,
		detail: TerrainDetail,
	) -> Self {
		if water_type.is_water() {
			if water_type == WaterType::Ocean {
				if flora_value > 0.7 {
					Self::Seaweed
				} else {
					Self::None
				}
			} else {
				Self::None
			}
		} else {
			match biome {
				Biome::Desert => {
					if flora_value > 0.8 {
						Self::Cactus
					} else if flora_value > 0.6 {
						Self::Bush
					} else {
						Self::None
					}
				}
				Biome::Forest => {
					if flora_value > 0.8 {
						Self::Tree
					} else if flora_value > 0.6 {
						Self::Bush
					} else if flora_value > 0.4 {
						Self::Flower
					} else {
						Self::None
					}
				}
				Biome::Jungle => {
					if flora_value > 0.7 {
						Self::Tree
					} else if flora_value > 0.5 {
						Self::Bush
					} else if flora_value > 0.3 {
						Self::Flower
					} else {
						Self::None
					}
				}
				Biome::Tundra => {
					if flora_value > 0.8 {
						Self::Bush
					} else if flora_value > 0.6 {
						Self::Flower
					} else {
						Self::None
					}
				}
				_ => {
					if flora_value > 0.8 {
						Self::Tree
					} else if flora_value > 0.6 {
						Self::Bush
					} else if flora_value > 0.4 {
						Self::Flower
					} else {
						Self::None
					}
				}
			}
		}
	}
}

pub struct FloraLayerFactory {
	noise_gen: NoiseGenerator,
}

impl FloraLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl
	LayerFactory<
		Flora,
		(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>, Layer<TerrainDetail>),
	> for FloraLayerFactory
{
	fn create_value(
		&self,
		pos: WorldPosition,
		deps: &(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>, Layer<TerrainDetail>),
	) -> Flora {
		let water_type = deps.0.get(pos);
		let terrain_feature = deps.1.get(pos);
		let biome = deps.2.get(pos);
		let detail = deps.3.get(pos);
		let value = self.noise_gen.get_noise_value(&pos, 4);
		Flora::from_values(
			value as f64 / u32::MAX as f64,
			water_type,
			terrain_feature,
			biome,
			detail,
		)
	}
}

pub fn generate_flora_layer(
	noise_gen: &NoiseGenerator,
	scale: u32,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
) -> Layer<Flora> {
	let factory = FloraLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer),
		factory,
		crate::layer::AllGridPositions::new(scale),
	)
}
