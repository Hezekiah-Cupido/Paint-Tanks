use avian3d::prelude::{Collider, CollisionStart, LinearVelocity, RigidBody};
use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    ecs::{
        children,
        component::Component,
        hierarchy::{ChildOf, Children},
        message::MessageReader,
        observer::On,
        query::With,
        relationship::RelatedSpawnerCommands,
        system::{Commands, Query, ResMut},
    },
    gltf::GltfAssetLabel,
    math::primitives::Sphere,
    mesh::Mesh,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{Mesh3d, SpawnRelated},
    scene::SceneRoot,
    transform::components::{GlobalTransform, Transform},
};

use crate::{
    entities::paintable_surface::PaintingObject, systems::despawn_entity::DespawnEntity, tank::Team,
};
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

// TODO: move to Turret component and use template design pattern for bullet mesh
fn shoot_bullet(
    mut shoot_event_reader: MessageReader<super::Shoot>,
    player_children: Query<(&Team, &Children), With<Player>>,
    turrets: Query<&Children, With<BasicTurret>>,
    bullet_spawner: Query<&GlobalTransform, With<super::BulletSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for event in shoot_event_reader.read() {
        if let Ok((team, children)) = player_children.get(event.entity)
            && let Some(turret_entity) = children
                .iter()
                .filter(|child| turrets.contains(**child))
                .nth(0)
            && let Ok(turret_children) = turrets.get(*turret_entity)
            && let Some(spawner_transform) = turret_children
                .into_iter()
                .filter_map(|t| bullet_spawner.get(*t).ok())
                .nth(0)
        {
            let bullet = meshes.add(Sphere::new(0.2));
            let bullet_material = materials.add(StandardMaterial {
                base_color: **team,
                ..Default::default()
            });

            commands
                .spawn((
                    Bullet::new(50),
                    PaintingObject::new(**team),
                    Mesh3d(bullet.clone()),
                    MeshMaterial3d(bullet_material.clone()),
                    Transform::from(spawner_transform.clone()),
                    Collider::sphere(0.2),
                    LinearVelocity(spawner_transform.forward() * BULLET_SPEED),
                ))
                .observe(
                    |collision_event: On<CollisionStart>,
                     mut commands: Commands<'_, '_>,
                     bullets: Query<&Bullet>,
                     mut players: Query<&mut Health, With<Player>>| {
                        let bullet_entity = collision_event.event().collider1;
                        let tank = collision_event.event().collider2;

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
