use bevy::{
    app::{App, Plugin, Update},
    math::Vec3,
    prelude::{Component, Query, Res, Transform},
    time::Time,
};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut position) in query.iter_mut() {
        position.translation += velocity.value * time.delta_seconds();
    }
}
