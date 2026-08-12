use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    ecs::{
        children,
        component::Component,
        entity::Entity,
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res},
    },
    transform::components::Transform,
    world_serialization::WorldAssetRoot,
};

use crate::{
    entities::paintable_surface::PaintableSurface, game_state::ClearWorld,
    systems::despawn_entity::DespawnEntity,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, despawn_map);
    }
}

#[derive(Component, Debug)]
pub struct Inactive;

#[derive(Component)]
#[require(Transform)]
pub struct SpawnPoint;

#[derive(Component, Default)]
struct Map;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = asset_server.load("models/plane_map.glb#Scene0");

    commands.spawn((
        Map,
        PaintableSurface,
        RigidBody::Static,
        Collider::cuboid(10., 0.5, 10.),
        Friction::new(0.9),
        Transform::from_xyz(0., 0., 0.),
        WorldAssetRoot(map),
        children![
            (SpawnPoint, Transform::from_xyz(0., 0.5, 0.)),
            (SpawnPoint, Transform::from_xyz(4., 0.5, 4.))
        ],
    ));
}

fn despawn_map(
    mut commands: Commands,
    clear_world_reader: MessageReader<ClearWorld>,
    maps: Query<Entity, With<Map>>,
) {
    if !clear_world_reader.is_empty() {
        for map_entity in maps.iter() {
            commands.entity(map_entity).insert(DespawnEntity);
        }
    }
}
