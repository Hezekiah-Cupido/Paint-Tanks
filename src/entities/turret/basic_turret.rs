use avian3d::prelude::{LinearVelocity, RigidBody};
use bevy::{
    app::{Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    ecs::{
        children,
        component::Component,
        event::EventReader,
        hierarchy::{ChildOf, Children},
        query::With,
        relationship::RelatedSpawnerCommands,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    gltf::GltfAssetLabel,
    math::primitives::Sphere,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::SpawnRelated,
    render::mesh::{Mesh, Mesh3d},
    scene::{Scene, SceneRoot},
    transform::components::{GlobalTransform, Transform},
};

use crate::entities::turret::{BulletSpawner, Turret, TurretSpawner};

trait BasicTurretSpawner {
    fn spawn_basic_turret(&mut self, world: &World);
}

#[derive(Resource)]
pub struct BasicTurretAsset(pub Option<Handle<Scene>>);

#[derive(Component)]
#[require(Turret)]
pub struct BasicTurret;

impl TurretSpawner for BasicTurret {
    fn spawn_turret(&self, commands: &mut RelatedSpawnerCommands<'_, ChildOf>, world: &World) {
        commands.spawn_basic_turret(world);
    }
}

impl BasicTurretSpawner for RelatedSpawnerCommands<'_, ChildOf> {
    fn spawn_basic_turret(&mut self, world: &World) {
        if let Some(turret_asset) = world.get_resource::<BasicTurretAsset>()
            && let Some(turret) = turret_asset.0.clone()
        {
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
}

pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Startup, load_asset)
        .add_systems(Update, shoot_bullet);
}

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let turret = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_turret.gltf"));

    commands.insert_resource(BasicTurretAsset(Some(turret)));
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
