use bevy::prelude::*;
use bevy_glyph::{GlyphBrushPlugin, Section, TextBrush};

const GRID_SIZE: usize = 16;
const CELL_SIZE: f32 = 40.0;
const GLYPH: &str = "🔲"; // Try 🔸, ✴️, 🧿, 🧩, ⬡, 🪙, etc.

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Procedural Grid".to_string(),
				resolution: (GRID_SIZE as f32 * CELL_SIZE, GRID_SIZE as f32 * CELL_SIZE).into(),
				..default()
			}),
			..default()
		}))
		.add_plugins(GlyphBrushPlugin)
		.add_startup_system(setup)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2dBundle::default());

	let font = asset_server.load("fonts/NotoEmoji-Regular.ttf"); // Good default for emoji
	let offset = CELL_SIZE / 2.0;

	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			let world_x = x as f32 * CELL_SIZE - (GRID_SIZE as f32 * CELL_SIZE / 2.0) + offset;
			let world_y = y as f32 * CELL_SIZE - (GRID_SIZE as f32 * CELL_SIZE / 2.0) + offset;

			commands.spawn(TextBrush::section(
				Section {
					screen_position: Vec2::new(world_x, world_y),
					text: vec![bevy_glyph::Text::new(GLYPH)
						.with_color(Color::WHITE)
						.with_scale(CELL_SIZE * 0.9)],
					..default()
				},
				font.clone(),
			));
		}
	}
}
