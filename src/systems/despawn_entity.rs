use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, despawn_entity);
}

#[derive(Component)]
pub struct DespawnEntity;

fn despawn_entity(mut commands: Commands, entities_to_despawn: Query<Entity, With<DespawnEntity>>) {
    for entity in entities_to_despawn.iter() {
        commands.entity(entity).despawn();
    }
}
