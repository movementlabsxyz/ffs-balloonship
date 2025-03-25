use crate::layer::base::NoiseGenerator;
use crate::layer::layers::biome::Biome;
use crate::layer::layers::detail::TerrainDetail;
use crate::layer::layers::flora::Flora;
use crate::layer::layers::terrain::TerrainFeature;
use crate::layer::layers::water::WaterType;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
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
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
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
			Urban::None => Color::NONE,
			Urban::House => Color::srgb(0.7, 0.7, 0.7),
			Urban::Farm => Color::srgb(0.8, 0.8, 0.6),
			Urban::City => Color::srgb(0.5, 0.5, 0.5),
			Urban::Port => Color::srgb(0.6, 0.6, 0.8),
			Urban::Mine => Color::srgb(0.4, 0.4, 0.4),
			Urban::Temple => Color::srgb(0.8, 0.8, 0.4),
			Urban::Ruin => Color::srgb(0.6, 0.6, 0.6),
		}
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
		pos: WorldPosition,
		deps: &(
			Layer<WaterType>,
			Layer<TerrainFeature>,
			Layer<Biome>,
			Layer<TerrainDetail>,
			Layer<Flora>,
		),
	) -> Urban {
		let water_type = deps.0.get(pos);
		let terrain_feature = deps.1.get(pos);
		let biome = deps.2.get(pos);
		let detail = deps.3.get(pos);
		let flora = deps.4.get(pos);
		let value = self.noise_gen.get_noise_value(&pos, 5);
		Urban::from_values(
			value as f64 / u32::MAX as f64,
			water_type,
			terrain_feature,
			biome,
			detail,
			flora,
		)
	}
}

pub fn generate_urban_layer(
	noise_gen: &NoiseGenerator,
	scale: u32,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
	flora_layer: Layer<Flora>,
) -> Layer<Urban> {
	let factory = UrbanLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer, flora_layer),
		factory,
		crate::layer::AllGridPositions::new(scale),
	)
}
