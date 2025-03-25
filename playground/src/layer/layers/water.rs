use crate::layer::{Layer, LayerFactory, LayerValue, WorldPosition};
use crate::terrain::base::NoiseGenerator;
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
	fn render(&self, commands: &mut Commands, screen_cell: &ScreenCell) {
		let color = self.get_color();

		if *self == WaterType::None {
			return;
		}

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

		// Calculate position for this grid cell
		let cell_size_f32 = CELL_SIZE as f32;
		let offset = cell_size_f32 / 2.0;
		let pos_x =
			(x as f32 * cell_size_f32) - ((GRID_SIZE as f32 * cell_size_f32) / 2.0) + offset;
		let pos_y =
			(y as f32 * cell_size_f32) - ((GRID_SIZE as f32 * cell_size_f32) / 2.0) + offset;

		// Spawn water sprite at grid cell scale
		commands.spawn((
			Sprite {
				color: self.get_color(),
				custom_size: Some(Vec2::new(cell_size_f32 - 2.0, cell_size_f32 - 2.0)),
				..default()
			},
			Transform::from_xyz(pos_x, pos_y, 0.0),
			..default(),
		));
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
	fn create_value(&self, pos: (usize, usize), _deps: &()) -> WaterType {
		let value = self.noise_gen.get_water_value(pos.0, pos.1);
		WaterType::from_value(value)
	}
}

pub fn generate_water_layer(noise_gen: &NoiseGenerator, scale: u64) -> Layer<WaterType> {
	let factory = WaterLayerFactory::new(noise_gen.clone());
	super::generate_layer(scale, (), factory, super::GridPositionIterator::new(scale))
}
