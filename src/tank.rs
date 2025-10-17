use avian3d::prelude::{AngularVelocity, Collider, Friction, LinearVelocity, Mass, RigidBody};
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res}, world::World,
    },
    gltf::GltfAssetLabel,
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    math::{primitives::InfinitePlane3d, Vec3},
    render::camera::Camera,
    scene::{Scene, SceneRoot},
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::{
    camera::MainCamera,
    entities::turret::{Shoot, Turret, TurretAsset, TurretMovement, basic_turret::BasicTurret},
};

const LINEAR_MOVEMENT_SPEED: f32 = 10.;
const ANGULAR_MOVEMENT_SPEED: f32 = 50.;

#[derive(Component)]
pub struct Tank;

#[derive(Resource)]
pub struct TankAssets {
    body: Option<Handle<Scene>>,
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

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Movement>()
        .add_systems(Startup, (load_tank_assets, spawn_tank).chain())
        .add_systems(
            Update,
            (keyboard_input, mouse_input, mouse_button_input, move_tank).chain(),
        );
}

fn load_tank_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let body = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_body.gltf"));

    commands.insert_resource(TurretAsset {
        turret: Box::new(BasicTurret {}),
    });
    commands.insert_resource(TankAssets { body: Some(body) });
}

fn spawn_tank(
    mut commands: Commands,
    world: &World,
    tank_assets: Res<TankAssets>,
    turret_asset: Res<TurretAsset>,
) {
    if let Some(body) = tank_assets.body.as_ref() {
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
                turret_asset.turret.spawn_turret(parent, world);
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
    player_children: Query<&Children, With<Player>>,
    turret_entities: Query<Entity, With<Turret>>,
) {
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera.single()
        && let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        && let Some(distance) =
            ray.intersect_plane(Vec3::new(0., 1., 0.), InfinitePlane3d::new(Vec3::Y))
        && let Ok(player_children) = player_children.single()
        && let Some(turret) = player_children
            .into_iter()
            .filter(|&c| turret_entities.as_readonly().get(*c).is_ok())
            .nth(0)
    {
        let point = ray.get_point(distance);

        turret_movemnt_event_writer.write(TurretMovement {
            turret_entity: *turret,
            x: point.x,
            z: point.z,
        });
    }
}

fn mouse_button_input(
    mut shoot_event_writer: EventWriter<Shoot>,
    input: Res<ButtonInput<MouseButton>>,
    player_children: Query<&Children, With<Player>>,
    turret_entities: Query<Entity, With<Turret>>,
) {
    if input.just_pressed(MouseButton::Left)
        && let Ok(player_children) = player_children.single()
        && let Some(turret) = player_children
            .into_iter()
            .filter(|&c| turret_entities.as_readonly().get(*c).is_ok())
            .nth(0)
    {
        shoot_event_writer.write(Shoot { turret: *turret });
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
