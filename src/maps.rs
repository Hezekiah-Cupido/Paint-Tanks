use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    ecs::{
        children,
        component::Component,
        system::{Commands, Res},
    },
    prelude::SpawnRelated,
    scene::SceneRoot,
    transform::components::Transform,
};

#[derive(Component)]
#[require(Transform::from_xyz(0., 0.5, 0.))]
pub struct SpawnPoint(pub bool);

#[derive(Component)]
pub struct Map;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_map);
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = asset_server.load("plane_map.glb#Scene0");

    commands.spawn((
        Map,
        RigidBody::Static,
        Collider::cuboid(10., 0.5, 10.),
        Friction::new(0.9),
        Transform::from_xyz(0., 0., 0.),
        SceneRoot(map),
        children![
            SpawnPoint(false),
            (SpawnPoint(false), Transform::from_xyz(4., 0.5, 4.))
        ],
    ));
}
