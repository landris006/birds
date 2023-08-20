use bevy::{prelude::*, window::WindowResolution};

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod components;
mod movement;
mod systems;

use movement::*;
use systems::*;

pub const BOUNDS: Vec2 = Vec2::new(1800., 900.);
pub const BIRDS_TO_SPAWN: i32 = 10;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                // mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                herbivore_flock_movement,
                herbivore_flee,
                carnivore_movement,
                keep_birds_in_bounds,
                move_birds_forward,
                draw_gizmos,
                energy_drain,
                zero_energy_dies,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
