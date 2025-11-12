use bevy::prelude::*;
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Floor {
    pub width: i32,
    pub height: i32,
    pub color: Color
}

#[derive(Component, Reflect)]
pub struct Loaded;

pub fn on_add_floor_component(
    trigger: On<Add, Floor>,
    mut commands: Commands,
    query: Query<(Entity, &Floor), Without<Loaded>>,
) {
    info!("Floor component added");
    info!("Floor details: {:?}", trigger.entity);

    for (entity, floor) in query.iter() {
        info!("Floor details: {:?}", floor);
        commands.entity(entity).insert(Loaded);
    }
}

