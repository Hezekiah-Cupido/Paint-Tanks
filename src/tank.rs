use bevy::{
    app::{App, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{Vec3, primitives::InfinitePlane3d},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::{
    camera::MainCamera,
    entities::{
        tank_body::{Movement, MovementType, TankBodySpawner, basic_tank_body::BasicTankBody},
        turret::{Shoot, Turret, TurretMovement, TurretSpawner, basic_turret::BasicTurret},
    },
    maps::SpawnPoint,
};

#[derive(Event)]
struct SpawnTank {
    player: Player,
    turret: Box<dyn TurretSpawner + Send + Sync>,
    tank_body: Box<dyn TankBodySpawner + Send + Sync>,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Player {
    User,
    Program,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnTank>().add_systems(
        Update,
        (
            spawn_tank_keyboard_input,
            spawn_tank,
            keyboard_input,
            mouse_input,
            mouse_button_input,
        ),
    );
}

fn spawn_tank(
    mut commands: Commands,
    mut spawn_tank_event_reader: EventReader<SpawnTank>,
    mut spawn_points: Query<(&mut SpawnPoint, &Transform), With<SpawnPoint>>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_tank_event_reader.read() {
        if let Some((mut spawn_point, transform)) =
            spawn_points.iter_mut().filter(|(s, _)| !s.0).nth(0)
        {
            spawn_point.0 = true;

            event
                .tank_body
                .spawn(&mut commands, &asset_server.as_ref())
                .insert(event.player)
                .insert(*transform)
                .with_children(|parent| {
                    event.turret.spawn_turret(parent, asset_server.as_ref());
                });
        }
    }
}

fn spawn_tank_keyboard_input(
    mut spawn_tank_event_writer: EventWriter<SpawnTank>,
    spawn_points: Query<&SpawnPoint, With<SpawnPoint>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let spawn_point_count = spawn_points.iter().filter(|s| !s.0).count();
        let player = if spawn_point_count == 2 {
            Player::User
        } else {
            Player::Program
        };

        spawn_tank_event_writer.write(SpawnTank {
            player: player,
            turret: Box::new(BasicTurret {}),
            tank_body: Box::new(BasicTankBody {}),
        });
    }
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<Movement>,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<(Entity, &Player), With<Player>>,
) {
    if let Some((entity, _)) = player.iter().filter(|(_, p)| **p == Player::User).nth(0) {
        let forward = input.any_pressed([KeyCode::KeyW]);
        let backward = input.any_pressed([KeyCode::KeyS]);
        let left = input.any_pressed([KeyCode::KeyA]);
        let right = input.any_pressed([KeyCode::KeyD]);

        let linear = forward as i8 - backward as i8;
        let angular = left as i8 - right as i8;

        movement_event_writer.write(Movement {
            entity: entity,
            movement_type: MovementType::Linear(linear),
        });

        movement_event_writer.write(Movement {
            entity: entity,
            movement_type: MovementType::Angular(angular),
        });
    }
}

fn mouse_input(
    mut turret_movemnt_event_writer: EventWriter<TurretMovement>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    player_children: Query<(&Children, &Player), With<Player>>,
    turret_entities: Query<Entity, With<Turret>>,
) {
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera.single()
        && let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        && let Some(distance) =
            ray.intersect_plane(Vec3::new(0., 1., 0.), InfinitePlane3d::new(Vec3::Y))
        && let Some((player_children, _)) = player_children
            .iter()
            .filter(|(_, p)| **p == Player::User)
            .nth(0)
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
    player_children: Query<(&Children, &Player), With<Player>>,
    turret_entities: Query<Entity, With<Turret>>,
) {
    if input.just_pressed(MouseButton::Left)
        && let Some((player_children, _)) = player_children
            .iter()
            .filter(|(_, p)| **p == Player::User)
            .nth(0)
        && let Some(turret) = player_children
            .into_iter()
            .filter(|&c| turret_entities.as_readonly().get(*c).is_ok())
            .nth(0)
    {
        shoot_event_writer.write(Shoot { turret: *turret });
    }
}
