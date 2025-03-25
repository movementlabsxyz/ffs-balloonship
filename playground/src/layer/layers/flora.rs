use super::biome::Biome;
use super::detail::TerrainDetail;
use super::terrain::TerrainFeature;
use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell};
use crate::terrain::base::NoiseGenerator;
use bevy::prelude::*;

#[derive(Clone, Copy, Default)]
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
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
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
					screen_cell.cell_size as f32,
					screen_cell.cell_size as f32,
				)),
				..default()
			},
			Transform::from_xyz(
				screen_cell.x as f32 * screen_cell.cell_size as f32,
				screen_cell.y as f32 * screen_cell.cell_size as f32,
				0.0,
			),
			..default(),
		));
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
		pos: (usize, usize),
		deps: &(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>, Layer<TerrainDetail>),
	) -> Flora {
		let water_type = deps.0.get(pos.0, pos.1);
		let terrain_feature = deps.1.get(pos.0, pos.1);
		let biome = deps.2.get(pos.0, pos.1);
		let detail = deps.3.get(pos.0, pos.1);
		let value = self.noise_gen.get_flora_value(pos.0, pos.1);
		Flora::from_values(value, water_type, terrain_feature, biome, detail)
	}
}

pub fn generate_flora_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
) -> Layer<Flora> {
	let factory = FloraLayerFactory::new(noise_gen.clone());
	super::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer),
		factory,
		super::GridPositionIterator::new(scale),
	)
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

	pub fn get_color(&self) -> Color {
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

	pub fn setup(
		&self,
		commands: &mut Commands,
		asset_server: &Res<AssetServer>,
		x: usize,
		y: usize,
	) {
		if self == &Self::None {
			return;
		}

		// Calculate position for this grid cell (at 64x64 scale)
		let offset = CELL_SIZE / 2.0;
		let pos_x = (x as f32 * CELL_SIZE / 4.0) - (GRID_SIZE as f32 * CELL_SIZE / 2.0) + offset;
		let pos_y = (y as f32 * CELL_SIZE / 4.0) - (GRID_SIZE as f32 * CELL_SIZE / 2.0) + offset;

		// Spawn flora sprite at quarter grid cell scale
		commands
			.spawn(Sprite {
				color: self.get_color(),
				custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
				..default()
			})
			.insert(Transform::from_xyz(pos_x, pos_y, 0.0));
	}
}
