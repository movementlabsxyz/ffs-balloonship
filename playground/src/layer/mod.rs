pub mod base;
pub mod layers;
use bevy::prelude::*;
use std::collections::HashMap;

/// A position relative to the entire world.
#[derive(Clone, Copy)]
pub struct WorldPosition {
	pub x: u32,
	pub y: u32,
}

/// A cell that gets filled in relative to the world.
#[derive(Clone, Copy)]
pub struct WorldCell {
	pub position: WorldPosition,
	pub cell_size: u32,
}

/// A value that can be rendered to a cell.
pub trait LayerValue: Clone + Copy + Default + PartialEq {
	fn render(&self, commands: &mut Commands, screen_cell: &WorldCell);
	fn get_color(&self) -> Color;
}

/// A factory that creates values for a layer.
pub trait LayerFactory<T: LayerValue, D> {
	fn create_value(&self, pos: WorldPosition, deps: &D) -> T;
}

/// A position relative to the grid, i.e., subdivisions of the world.
#[derive(Clone, Copy)]
pub struct GridPosition {
	pub x: u32,
	pub y: u32,
	pub scale: u32,
}

impl GridPosition {
	pub fn new(x: u32, y: u32, scale: u32) -> Self {
		Self { x, y, scale }
	}
}

/// You can always iterate over a grid by going up to the scale.
impl Iterator for GridPosition {
	type Item = GridPosition;

	fn next(&mut self) -> Option<Self::Item> {
		self.x += 1;
		if self.x >= self.scale {
			self.x = 0;
			self.y += 1;
		}

		if self.y >= self.scale {
			return None;
		}

		Some(*self)
	}
}

impl From<GridPosition> for WorldPosition {
	fn from(grid_position: GridPosition) -> Self {
		WorldPosition {
			x: grid_position.x * grid_position.scale,
			y: grid_position.y * grid_position.scale,
		}
	}
}

/// A layer contains a grid of values.
pub struct Layer<T: LayerValue> {
	data: HashMap<GridPosition, T>,
	scale: u32,
}

impl<T: LayerValue> Layer<T> {
	pub fn new_base32(scale_factor: u32) -> Self {
		let scale = 1 << scale_factor;
		Self { data: HashMap::new(), scale }
	}

	/// Gets the grid position for the given [WorldPosition].
	pub fn get_grid_position(&self, position: WorldPosition) -> GridPosition {
		GridPosition { x: position.x / self.scale, y: position.y / self.scale, scale: self.scale }
	}

	/// Get the value at the given [GridPosition].
	pub fn get_grid(&self, position: GridPosition) -> T {
		self.data.get(&position).copied().unwrap_or_default()
	}

	/// Get the value at the given [WorldPosition].
	pub fn get(&self, position: WorldPosition) -> T {
		let grid_position = self.get_grid_position(position);
		self.get_grid(grid_position)
	}

	/// Set the value at the given [GridPosition].
	pub fn set_grid(&mut self, position: GridPosition, value: T) {
		if value != T::default() {
			self.data.insert(position, value);
		}
	}

	/// Set the value at the given [WorldPosition].
	pub fn set(&mut self, position: WorldPosition, value: T) {
		let grid_position = self.get_grid_position(position);
		self.set_grid(grid_position, value);
	}

	/// Get the scale of the layer.
	pub fn scale(&self) -> u32 {
		self.scale
	}

	/// Render the layer to the given [Commands].
	pub fn render(&self, commands: &mut Commands) {
		for (&(x, y), value) in &self.data {
			value.render(commands, &WorldCell { x, y, cell_size: self.scale });
		}
	}
}

/// An iterator over all grid positions in the layer.
pub struct AllGridPositions {
	pub grid_position: GridPosition,
}

impl AllGridPositions {
	pub fn new(scale: u32) -> Self {
		Self { grid_position: GridPosition::new(0, 0, scale) }
	}
}

impl Iterator for AllGridPositions {
	type Item = WorldPosition;

	// Simply iterate over the grid positions and convert them to world positions.
	fn next(&mut self) -> Option<Self::Item> {
		self.grid_position.next().map(|grid_position| grid_position.into())
	}
}

pub fn generate_layer<T: LayerValue, D, F: LayerFactory<T, D>>(
	scale_factor: u32,
	deps: D,
	factory: F,
	positions: impl Iterator<Item = WorldPosition>,
) -> Layer<T> {
	let mut layer = Layer::new_base32(scale_factor);
	for pos in positions {
		let value = factory.create_value(pos, &deps);
		layer.set(pos, value);
	}
	layer
}
