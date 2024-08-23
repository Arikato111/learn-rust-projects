use bevy::prelude::*;
use space_ship::{CameraPlugin, DebugPlugin, MovementPlugin, SpaceshipPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
