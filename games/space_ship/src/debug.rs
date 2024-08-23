use bevy::{
    app::{App, Plugin, Update},
    log::info,
    prelude::{Entity, Query, Transform},
};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, tranfrom) in query.iter() {
        info!("Entity {:?} is position {:?}", entity, tranfrom);
    }
}
