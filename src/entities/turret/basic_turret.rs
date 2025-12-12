use avian3d::prelude::{
    Collider, CollisionEventsEnabled, LinearVelocity, OnCollisionStart, RigidBody,
};
use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        children,
        component::Component,
        event::EventReader,
        hierarchy::{ChildOf, Children},
        observer::Trigger,
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

use crate::systems::despawn_entity::DespawnEntity;
use crate::{
    entities::{
        bullet::Bullet,
        turret::{BulletSpawner, Turret, TurretSpawner},
    },
    tank::{Health, Player},
};

pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, shoot_bullet);
}

const BULLET_SPEED: f32 = 20.;

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

            commands
                .spawn((
                    Bullet::new(50),
                    Mesh3d(bullet.clone()),
                    MeshMaterial3d(bullet_material.clone()),
                    Transform::from(spawner_transform.clone()),
                    RigidBody::Dynamic,
                    Collider::sphere(0.2),
                    CollisionEventsEnabled,
                    LinearVelocity(spawner_transform.forward() * BULLET_SPEED),
                ))
                .observe(
                    |trigger: Trigger<OnCollisionStart>,
                     mut commands: Commands<'_, '_>,
                     bullets: Query<&Bullet>,
                     mut players: Query<&mut Health, With<Player>>| {
                        let bullet_entity = trigger.target();
                        let tank = trigger.collider;

                        if let Ok(mut player_health) = players.get_mut(tank)
                            && let Ok(bullet) = bullets.get(bullet_entity)
                        {
                            player_health.0 = player_health.0.saturating_sub(bullet.damage);
                            println!("Hit: {}", player_health.0.clone());

                            if player_health.0 == 0 {
                                commands.entity(tank).insert(DespawnEntity);
                            }

                            commands.entity(bullet_entity).insert(DespawnEntity);
                        }
                    },
                );
        }
    }
}
