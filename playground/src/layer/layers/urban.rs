use super::biome::Biome;
use super::detail::TerrainDetail;
use super::flora::Flora;
use super::terrain::TerrainFeature;
use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell};
use crate::terrain::base::NoiseGenerator;
use bevy::prelude::*;

#[derive(Clone, Copy, Default)]
pub enum Urban {
	#[default]
	None,
	House,
	Farm,
	City,
	Port,
	Mine,
	Temple,
	Ruin,
}

impl LayerValue for Urban {
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
		let color = match self {
			Urban::None => return,
			Urban::House => Color::rgb(0.7, 0.7, 0.7),
			Urban::Farm => Color::rgb(0.8, 0.8, 0.6),
			Urban::City => Color::rgb(0.5, 0.5, 0.5),
			Urban::Port => Color::rgb(0.6, 0.6, 0.8),
			Urban::Mine => Color::rgb(0.4, 0.4, 0.4),
			Urban::Temple => Color::rgb(0.8, 0.8, 0.4),
			Urban::Ruin => Color::rgb(0.6, 0.6, 0.6),
		};

		commands.spawn((SpriteBundle {
			sprite: Sprite {
				color,
				custom_size: Some(Vec2::new(
					screen_cell.cell_size as f32,
					screen_cell.cell_size as f32,
				)),
				..default()
			},
			transform: Transform::from_xyz(
				screen_cell.x as f32 * screen_cell.cell_size as f32,
				screen_cell.y as f32 * screen_cell.cell_size as f32,
				0.0,
			),
			..default()
		},));
	}
}

impl Urban {
	pub fn from_values(
		urban_value: f64,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
		biome: Biome,
		detail: TerrainDetail,
		flora: Flora,
	) -> Self {
		if water_type.is_water() {
			if urban_value > 0.8 {
				Self::Port
			} else {
				Self::None
			}
		} else {
			match terrain_feature {
				TerrainFeature::Mountain => {
					if urban_value > 0.7 {
						Self::Mine
					} else {
						Self::None
					}
				}
				TerrainFeature::Plains => {
					if urban_value > 0.9 {
						Self::City
					} else if urban_value > 0.7 {
						Self::Farm
					} else if urban_value > 0.5 {
						Self::House
					} else {
						Self::None
					}
				}
				_ => Self::None,
			}
		}
	}
}

pub struct UrbanLayerFactory {
	noise_gen: NoiseGenerator,
}

impl UrbanLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl
	LayerFactory<
		Urban,
		(Layer<WaterType>, Layer<TerrainFeature>, Layer<Biome>, Layer<TerrainDetail>, Layer<Flora>),
	> for UrbanLayerFactory
{
	fn create_value(
		&self,
		pos: (usize, usize),
		deps: &(
			Layer<WaterType>,
			Layer<TerrainFeature>,
			Layer<Biome>,
			Layer<TerrainDetail>,
			Layer<Flora>,
		),
	) -> Urban {
		let water_type = deps.0.get(pos.0, pos.1);
		let terrain_feature = deps.1.get(pos.0, pos.1);
		let biome = deps.2.get(pos.0, pos.1);
		let detail = deps.3.get(pos.0, pos.1);
		let flora = deps.4.get(pos.0, pos.1);
		let value = self.noise_gen.get_urban_value(pos.0, pos.1);
		Urban::from_values(value, water_type, terrain_feature, biome, detail, flora)
	}
}

pub fn generate_urban_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
	flora_layer: Layer<Flora>,
) -> Layer<Urban> {
	let factory = UrbanLayerFactory::new(noise_gen.clone());
	super::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer, flora_layer),
		factory,
		super::GridPositionIterator::new(scale),
	)
}
