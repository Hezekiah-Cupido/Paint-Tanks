use avian3d::prelude::{LinearVelocity, RigidBody};
use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        children,
        component::Component,
        event::EventReader,
        hierarchy::{ChildOf, Children},
        query::With,
        relationship::RelatedSpawnerCommands,
        system::{Commands, Query, ResMut},
    },
    gltf::GltfAssetLabel,
    math::primitives::Sphere,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::SpawnRelated,
    render::mesh::{Mesh, Mesh3d},
    scene::SceneRoot,
    transform::components::{GlobalTransform, Transform},
};

use crate::entities::turret::{BulletSpawner, Turret, TurretSpawner};

trait BasicTurretSpawner {
    fn spawn_basic_turret(&mut self, asset_server: &AssetServer);
}

#[derive(Component)]
#[require(Turret)]
pub struct BasicTurret;

impl TurretSpawner for BasicTurret {
    fn spawn_turret(
        &self,
        commands: &mut RelatedSpawnerCommands<'_, ChildOf>,
        asset_server: &AssetServer,
    ) {
        commands.spawn_basic_turret(asset_server);
    }
}

impl BasicTurretSpawner for RelatedSpawnerCommands<'_, ChildOf> {
    fn spawn_basic_turret(&mut self, asset_server: &AssetServer) {
        let turret = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_turret.gltf"));

        self.spawn((
            BasicTurret,
            Transform::from_xyz(0., 0.5, 0.),
            SceneRoot(turret),
            children![(
                BulletSpawner,
                RigidBody::Kinematic,
                Transform::from_xyz(0., 0.25, -1.),
            )],
        ));
    }
}

pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, shoot_bullet);
}

fn shoot_bullet(
    mut shoot_event_reader: EventReader<super::Shoot>,
    turrets: Query<&Children, With<BasicTurret>>,
    bullet_spawner: Query<&GlobalTransform, With<super::BulletSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for event in shoot_event_reader.read() {
        if let Ok(turret_children) = turrets.get(event.turret)
            && let Some(spawner_transform) = turret_children
                .into_iter()
                .filter_map(|t| bullet_spawner.get(*t).ok())
                .nth(0)
        {
            let bullet = meshes.add(Sphere::new(0.2));
            let bullet_material = materials.add(StandardMaterial {
                base_color: Color::srgba(1., 0.0, 0.0, 1.0),
                ..Default::default()
            });

            commands.spawn((
                RigidBody::Dynamic,
                Mesh3d(bullet.clone()),
                MeshMaterial3d(bullet_material.clone()),
                Transform::from(spawner_transform.clone()),
                LinearVelocity(spawner_transform.forward().into()),
            ));
        }
    }
}
