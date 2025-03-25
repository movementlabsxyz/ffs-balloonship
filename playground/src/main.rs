use bevy::prelude::*;
pub mod layer;

#[derive(Component)]
struct GridLine;

fn main() {
	App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Camera
	commands.spawn((Camera2d, Transform::default()));

	// Background
	commands.spawn((
		Sprite {
			color: Color::srgb(0.1, 0.1, 0.1),
			custom_size: Some(Vec2::new(2000.0, 2000.0)),
			..default()
		},
		Transform::default(),
	));

	// Initialize noise generator
	let noise_gen = NoiseGenerator::new();

	// Render layers in order from bottom to top with proper scales
	// Base layers (scale 60 = 16x16 grid)
	render_layer_grid(generate_water_layer(&noise_gen, 60), &mut commands);
	render_layer_grid(generate_terrain_layer(&noise_gen, 60), &mut commands);
	render_layer_grid(generate_biome_layer(&noise_gen, 60), &mut commands);

	// Detail layers (scale 58 = 64x64 grid)
	render_layer_grid(generate_detail_layer(&noise_gen, 58), &mut commands);
	render_layer_grid(generate_flora_layer(&noise_gen, 58), &mut commands);
	render_layer_grid(generate_urban_layer(&noise_gen, 58), &mut commands);

	// Special layer (scale 56 = 256x256 grid)
	render_layer_grid(generate_special_layer(&noise_gen, 56), &mut commands);

	// Draw grid lines
	for i in 0..=GRID_SIZE {
		// Vertical lines
		commands.spawn((
			Sprite {
				color: Color::srgba(1.0, 1.0, 1.0, 0.1),
				custom_size: Some(Vec2::new(2.0, GRID_SIZE as f32 * CELL_SIZE)),
				..default()
			},
			Transform::from_xyz(
				(i as f32 * CELL_SIZE) - (GRID_SIZE as f32 * CELL_SIZE / 2.0),
				0.0,
				0.0,
			),
			GridLine,
		));

		// Horizontal lines
		commands.spawn((
			Sprite {
				color: Color::srgba(1.0, 1.0, 1.0, 0.1),
				custom_size: Some(Vec2::new(GRID_SIZE as f32 * CELL_SIZE, 2.0)),
				..default()
			},
			Transform::from_xyz(
				0.0,
				(i as f32 * CELL_SIZE) - (GRID_SIZE as f32 * CELL_SIZE / 2.0),
				0.0,
			),
			GridLine,
		));
	}
}
