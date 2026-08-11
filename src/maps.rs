use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::{
    asset::AssetServer,
    ecs::{
        children,
        component::Component,
        system::{Commands, Res},
    },
    transform::components::Transform,
    world_serialization::WorldAssetRoot,
};

use crate::entities::paintable_surface::PaintableSurface;

#[derive(Component, Debug)]
pub struct Inactive;

#[derive(Component)]
#[require(Transform)]
pub struct SpawnPoint;

#[derive(Component)]
pub struct Map;

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
