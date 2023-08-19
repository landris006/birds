use bevy::prelude::*;
mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
