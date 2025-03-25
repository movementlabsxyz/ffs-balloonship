use super::biome::Biome;
use super::detail::TerrainDetail;
use super::flora::Flora;
use super::terrain::TerrainFeature;
use super::urban::Urban;
use super::water::WaterType;
use super::{Layer, LayerFactory, LayerValue, ScreenCell};
use crate::terrain::base::NoiseGenerator;
use bevy::prelude::*;

#[derive(Clone, Copy, Default)]
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
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
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
		pos: (usize, usize),
		deps: &(
			Layer<WaterType>,
			Layer<TerrainFeature>,
			Layer<Biome>,
			Layer<TerrainDetail>,
			Layer<Flora>,
			Layer<Urban>,
		),
	) -> Special {
		let water_type = deps.0.get(pos.0, pos.1);
		let terrain_feature = deps.1.get(pos.0, pos.1);
		let biome = deps.2.get(pos.0, pos.1);
		let detail = deps.3.get(pos.0, pos.1);
		let flora = deps.4.get(pos.0, pos.1);
		let urban = deps.5.get(pos.0, pos.1);
		let value = self.noise_gen.get_special_value(pos.0, pos.1);
		Special::from_values(value, water_type, terrain_feature, biome, detail, flora, urban)
	}
}

pub fn generate_special_layer(
	noise_gen: &NoiseGenerator,
	scale: u64,
	water_layer: Layer<WaterType>,
	terrain_layer: Layer<TerrainFeature>,
	biome_layer: Layer<Biome>,
	detail_layer: Layer<TerrainDetail>,
	flora_layer: Layer<Flora>,
	urban_layer: Layer<Urban>,
) -> Layer<Special> {
	let factory = SpecialLayerFactory::new(noise_gen.clone());
	super::generate_layer(
		scale,
		(water_layer, terrain_layer, biome_layer, detail_layer, flora_layer, urban_layer),
		factory,
		super::GridPositionIterator::new(scale),
	)
}
