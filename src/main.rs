use bevy::{prelude::*, window::WindowMode};
mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // resolution: WindowResolution::new(1920., 1080.),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_birds,
                bevy::window::close_on_esc,
                draw_gizmos,
                rotate_birds,
            ),
        )
        .run();
}
