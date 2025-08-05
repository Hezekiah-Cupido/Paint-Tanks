use avian3d::{
    math::PI,
    prelude::{AngularVelocity, Collider, Friction, LinearVelocity, Mass, RigidBody},
};
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        query::{With, Without},
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    gltf::GltfAssetLabel,
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{
        Vec3, Vec3Swizzles,
        ops::acos,
        primitives::{InfinitePlane3d, Sphere},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    render::{
        camera::Camera,
        mesh::{Mesh, Mesh3d},
    },
    scene::{Scene, SceneRoot},
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::camera::MainCamera;

const LINEAR_MOVEMENT_SPEED: f32 = 10.;
const ANGULAR_MOVEMENT_SPEED: f32 = 50.;
const TURRET_ROTATION_SPEED: f32 = 3.;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct BulletSpawner;

#[derive(Resource)]
pub struct TankAssets {
    body: Option<Handle<Scene>>,
    turret: Option<Handle<Scene>>,
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
struct Movement {
    entity: Entity,
    movement_type: MovementType,
}

enum MovementType {
    Linear(i8),
    Angular(i8),
}

#[derive(Event)]
struct TurretMovement {
    entity: Entity,
    x: f32,
    z: f32,
}

#[derive(Event)]
struct Shoot {
    entity: Entity,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (load_tank_assets, spawn_tank).chain())
        .add_event::<Movement>()
        .add_event::<TurretMovement>()
        .add_event::<Shoot>()
        .add_systems(
            Update,
            (
                keyboard_input,
                mouse_input,
                mouse_button_input,
                move_tank,
                move_turret,
                shoot_bullet,
            )
                .chain(),
        );
}

fn load_tank_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let turret = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_turret.gltf"));
    let body = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_body.gltf"));

    commands.insert_resource(TankAssets {
        body: Some(body),
        turret: Some(turret),
    });
}

fn spawn_tank(mut commands: Commands, tank_assets: Res<TankAssets>) {
    if let Some(body) = tank_assets.body.as_ref()
        && let Some(turret) = tank_assets.turret.as_ref()
    {
        commands
            .spawn((
                Tank,
                Player,
                RigidBody::Dynamic,
                Collider::cuboid(1., 1., 1.),
                Mass(100.),
                Friction::new(0.9),
                Transform::from_xyz(0., 1., 0.),
                SceneRoot(body.clone()),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Turret,
                        Transform::from_xyz(0., 0.5, 0.),
                        SceneRoot(turret.clone()),
                    ))
                    .with_child((
                        BulletSpawner,
                        RigidBody::Kinematic,
                        Transform::from_xyz(0., 0.25, -1.),
                    ));
            });
    }
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<Movement>,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<Entity, With<Player>>,
) {
    if let Ok(player) = player.single() {
        let forward = input.any_pressed([KeyCode::KeyW]);
        let backward = input.any_pressed([KeyCode::KeyS]);
        let left = input.any_pressed([KeyCode::KeyA]);
        let right = input.any_pressed([KeyCode::KeyD]);

        let linear = forward as i8 - backward as i8;
        let angular = left as i8 - right as i8;

        movement_event_writer.write(Movement {
            entity: player,
            movement_type: MovementType::Linear(linear),
        });

        movement_event_writer.write(Movement {
            entity: player,
            movement_type: MovementType::Angular(angular),
        });
    }
}

fn mouse_input(
    mut turret_movemnt_event_writer: EventWriter<TurretMovement>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    player: Query<Entity, With<Player>>,
) {
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera.single()
        && let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        && let Some(distance) =
            ray.intersect_plane(Vec3::new(0., 1., 0.), InfinitePlane3d::new(Vec3::Y))
        && let Ok(player) = player.single()
    {
        let point = ray.get_point(distance);

        turret_movemnt_event_writer.write(TurretMovement {
            entity: player,
            x: point.x,
            z: point.z,
        });
    }
}

fn mouse_button_input(
    mut shoot_event_writer: EventWriter<Shoot>,
    input: Res<ButtonInput<MouseButton>>,
    player: Query<Entity, With<Player>>,
) {
    if input.just_pressed(MouseButton::Left)
        && let Ok(player) = player.single()
    {
        shoot_event_writer.write(Shoot { entity: player });
    }
}

fn move_tank(
    mut movement_event_reader: EventReader<Movement>,
    mut tanks: Query<(&mut LinearVelocity, &mut AngularVelocity, &Transform), With<Tank>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for event in movement_event_reader.read() {
        if let Ok((mut linear_velocity, mut angular_velocity, transform)) =
            tanks.get_mut(event.entity)
        {
            match event.movement_type {
                MovementType::Linear(linear_amount) => {
                    linear_velocity.z += transform.forward().z
                        * (linear_amount as f32)
                        * delta_time
                        * LINEAR_MOVEMENT_SPEED;
                    linear_velocity.x += transform.forward().x
                        * (linear_amount as f32)
                        * delta_time
                        * LINEAR_MOVEMENT_SPEED;
                }
                MovementType::Angular(angular_amount) => {
                    angular_velocity.y +=
                        angular_amount as f32 * delta_time * ANGULAR_MOVEMENT_SPEED;
                }
            }
        }
    }
}

fn move_turret(
    mut turret_movement_event_reader: EventReader<TurretMovement>,
    tanks: Query<&Children, With<Tank>>,
    mut turret_transforms: Query<(&mut Transform, &GlobalTransform), (With<Turret>, Without<Tank>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for event in turret_movement_event_reader.read() {
        if let Ok(tank_children) = tanks.get(event.entity)
            && let Some(turret_entity) = tank_children
                .into_iter()
                .filter(|&c| turret_transforms.as_readonly().get(*c).is_ok())
                .nth(0)
        {
            if let Ok((mut turret_transform, turret_global_transform)) =
                turret_transforms.get_mut(*turret_entity)
            {
                let turret_translation = turret_transform.translation.clone();

                let x = event.x;
                let y = turret_translation.y;
                let z = event.z;

                let to_cursor = (Vec3::new(x, y, z) - turret_translation).normalize();

                let _turret_rotation = turret_transform.rotation.clone();

                let turret_rotation_x = (turret_global_transform.rotation() * Vec3::X).normalize();
                let turret_rotation_y = turret_global_transform.forward().normalize();

                let rotation_angle =
                    acos(turret_rotation_y.xz().dot(to_cursor.xz()).clamp(-1., 1.));

                if rotation_angle - (PI / 180.) > f32::EPSILON {
                    let rotation_sign = -f32::copysign(1., turret_rotation_x.dot(to_cursor));

                    let turret_rotation_rate: f32 =
                        (TURRET_ROTATION_SPEED / rotation_angle).clamp(1., TURRET_ROTATION_SPEED);
                    turret_transform.rotate_y(
                        rotation_sign * rotation_angle * turret_rotation_rate * delta_time,
                    );
                }
            }
        }
    }
}

fn shoot_bullet(
    mut shoot_event_reader: EventReader<Shoot>,
    tanks: Query<&Children, With<Tank>>,
    turrets: Query<&Children, (With<Turret>, Without<Tank>)>,
    bullet_spawner: Query<&GlobalTransform, With<BulletSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for event in shoot_event_reader.read() {
        if let Ok(tank_children) = tanks.get(event.entity)
            && let Some(turret_children) = tank_children
                .into_iter()
                .filter_map(|t| turrets.get(*t).ok())
                .nth(0)
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
