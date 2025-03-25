use crate::layer::base::NoiseGenerator;
use crate::layer::{Layer, LayerFactory, LayerValue, WorldCell, WorldPosition};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum WaterType {
	#[default]
	None,
	Ocean,
	Lake,
	River,
	Swamp,
}

impl LayerValue for WaterType {
	fn render(&self, commands: &mut Commands, world_cell: &WorldCell) {
		let color = self.get_color();

		if *self == WaterType::None {
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
			Transform::from_xyz(
				world_cell.position.x as f32 * world_cell.cell_size as f32,
				world_cell.position.y as f32 * world_cell.cell_size as f32,
				0.0,
			),
		));
	}

	fn get_color(&self) -> Color {
		match self {
			WaterType::None => Color::NONE,
			WaterType::Ocean => Color::srgb(0.0, 0.2, 0.8),
			WaterType::Lake => Color::srgb(0.0, 0.4, 0.8),
			WaterType::River => Color::srgb(0.0, 0.6, 0.8),
			WaterType::Swamp => Color::srgb(0.0, 0.5, 0.3),
		}
	}
}

impl WaterType {
	pub fn from_value(water_value: f64) -> Self {
		if water_value > 0.8 {
			Self::Ocean
		} else if water_value > 0.6 {
			Self::Lake
		} else if water_value > 0.4 {
			Self::River
		} else if water_value > 0.2 {
			Self::Swamp
		} else {
			Self::None
		}
	}

	pub fn is_water(&self) -> bool {
		*self != Self::None
	}
}

pub struct WaterLayerFactory {
	noise_gen: NoiseGenerator,
}

impl WaterLayerFactory {
	pub fn new(noise_gen: NoiseGenerator) -> Self {
		Self { noise_gen }
	}
}

impl LayerFactory<WaterType, ()> for WaterLayerFactory {
	fn create_value(&self, pos: WorldPosition, _deps: &()) -> WaterType {
		let value = self.noise_gen.get_noise_value(&pos, 0);
		WaterType::from_value(value as f64 / u32::MAX as f64)
	}
}

pub fn generate_water_layer(noise_gen: &NoiseGenerator, scale: u32) -> Layer<WaterType> {
	let factory = WaterLayerFactory::new(noise_gen.clone());
	crate::layer::generate_layer(scale, (), factory, crate::layer::AllGridPositions::new(scale))
}
