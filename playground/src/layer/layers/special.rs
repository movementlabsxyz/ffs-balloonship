use crate::layer::base::NoiseGenerator;
use crate::layer::layers::biome::Biome;
use crate::layer::layers::detail::TerrainDetail;
use crate::layer::layers::flora::Flora;
use crate::layer::layers::terrain::TerrainFeature;
use crate::layer::layers::urban::Urban;
use crate::layer::layers::water::WaterType;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Special {
	#[default]
	None,
	Volcano,
	Geyser,
	Crystal,
	Portal,
	Ruins,
	Temple,
	Dungeon,
}

impl LayerValue for Special {
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
		let color = match self {
			Special::None => return,
			Special::Volcano => Color::rgb(0.8, 0.2, 0.0),
			Special::Geyser => Color::rgb(0.0, 0.8, 0.8),
			Special::Crystal => Color::rgb(0.8, 0.8, 1.0),
			Special::Portal => Color::rgb(0.8, 0.0, 0.8),
			Special::Ruins => Color::rgb(0.6, 0.6, 0.6),
			Special::Temple => Color::rgb(0.8, 0.8, 0.6),
			Special::Dungeon => Color::rgb(0.4, 0.4, 0.4),
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
			Special::None => Color::NONE,
			Special::Volcano => Color::srgb(0.8, 0.2, 0.0),
			Special::Geyser => Color::srgb(0.0, 0.8, 0.8),
			Special::Crystal => Color::srgb(0.8, 0.8, 1.0),
			Special::Portal => Color::srgb(0.8, 0.0, 0.8),
			Special::Ruins => Color::srgb(0.6, 0.6, 0.6),
			Special::Temple => Color::srgb(0.8, 0.8, 0.6),
			Special::Dungeon => Color::srgb(0.4, 0.4, 0.4),
		}
	}
}

impl Special {
	pub fn from_values(
		special_value: f64,
		water_type: WaterType,
		terrain_feature: TerrainFeature,
		biome: Biome,
		detail: TerrainDetail,
		flora: Flora,
		urban: Urban,
	) -> Self {
		if special_value > 0.95 {
			match terrain_feature {
				TerrainFeature::Mountain => Self::Volcano,
				_ if water_type.is_water() => Self::Geyser,
				_ => Self::Crystal,
			}
		} else if special_value > 0.9 {
			match urban {
				Urban::Temple => Self::Temple,
				Urban::Ruin => Self::Ruins,
				_ => Self::Portal,
			}
		} else if special_value > 0.85 {
			Self::Dungeon
		} else {
			Self::None
		}
	}
}

pub struct SpecialLayerFactory {
	noise_gen: NoiseGenerator,
}

impl SpecialLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl
	LayerFactory<
		Special,
		(
			Layer<WaterType>,
			Layer<TerrainFeature>,
			Layer<Biome>,
			Layer<TerrainDetail>,
			Layer<Flora>,
			Layer<Urban>,
		),
	> for SpecialLayerFactory
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
			Layer<Urban>,
		),
	) -> Special {
		let water_type = deps.0.get(pos);
		let terrain_feature = deps.1.get(pos);
		let biome = deps.2.get(pos);
		let detail = deps.3.get(pos);
		let flora = deps.4.get(pos);
		let urban = deps.5.get(pos);
		let value = self.noise_gen.get_noise_value(&pos, 6);
		Special::from_values(
			value as f64 / u32::MAX as f64,
			water_type,
			terrain_feature,
			biome,
			detail,
			flora,
			urban,
		)
	}
}

pub fn generate_special_layer(
	noise_gen: &NoiseGenerator,
	scale: u32,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
	flora_layer: Layer<Flora>,
	urban_layer: Layer<Urban>,
) -> Layer<Special> {
	let factory = SpecialLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer, flora_layer, urban_layer),
		factory,
		crate::layer::AllGridPositions::new(scale),
	)
}
