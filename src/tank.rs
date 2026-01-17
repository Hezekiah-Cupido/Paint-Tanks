use bevy::{
    app::{App, Update},
    asset::AssetServer,
    camera::Camera,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        message::{Message, MessageReader, MessageWriter},
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{Vec3, primitives::InfinitePlane3d},
    prelude::Deref,
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::{
    camera::MainCamera,
    entities::{
        tank_body::{
            self, Movement, MovementType, TankBodySpawner, basic_tank_body::BasicTankBody,
        },
        turret::{self, Shoot, TurretMovement, TurretSpawner, basic_turret::BasicTurret},
    },
    maps::{Inactive, SpawnPoint},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((turret::plugin, tank_body::plugin))
        .add_message::<SpawnTank>()
        .add_systems(
            Update,
            (
                spawn_tank_keyboard_input,
                spawn_tank,
                keyboard_input,
                move_turret_mouse_input,
                shoot_mouse_input,
            ),
        );
}

#[derive(Message)]
struct SpawnTank {
    player: Player,
    team: Team,
    turret: Box<dyn TurretSpawner + Send + Sync>,
    tank_body: Box<dyn TankBodySpawner + Send + Sync>,
}

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component, Clone, Copy, Debug, Deref)]
pub struct Team(pub Color);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[require(Health(100), Team(Color::srgba(1., 0.0, 0.0, 1.0)))]
pub enum Player {
    User,
    Program,
}

fn spawn_tank(
    mut commands: Commands,
    mut spawn_tank_event_reader: MessageReader<SpawnTank>,
    spawn_points: Query<&Transform, Without<Inactive>>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_tank_event_reader.read() {
        if let Some(transform) = spawn_points.iter().nth(0) {
            event
                .tank_body
                .spawn(&mut commands, &asset_server.as_ref())
                .insert((event.player, event.team, *transform))
                .with_children(|parent| {
                    event.turret.spawn_turret(parent, asset_server.as_ref());
                });
        }
    }
}

fn spawn_tank_keyboard_input(
    mut spawn_tank_event_writer: MessageWriter<SpawnTank>,
    spawn_points: Query<&SpawnPoint, Without<Inactive>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let spawn_point_count = spawn_points.iter().count();
        let (player, team) = if spawn_point_count == 2 {
            (Player::User, Team(Color::srgb(1., 0., 0.)))
        } else {
            (Player::Program, Team(Color::BLACK))
        };

        spawn_tank_event_writer.write(SpawnTank {
            player: player,
            team: team,
            turret: Box::new(BasicTurret {}),
            tank_body: Box::new(BasicTankBody {}),
        });
    }
}

fn keyboard_input(
    mut movement_event_writer: MessageWriter<Movement>,
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

fn move_turret_mouse_input(
    mut turret_movemnt_event_writer: MessageWriter<TurretMovement>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    players: Query<(Entity, &Player)>,
) {
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera.single()
        && let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        && let Some(distance) =
            ray.intersect_plane(Vec3::new(0., 1., 0.), InfinitePlane3d::new(Vec3::Y))
    {
        let player_entities = players
            .iter()
            .filter(|(_, player)| **player == Player::User);

        for (entity, _) in player_entities {
            let point = ray.get_point(distance);

            turret_movemnt_event_writer.write(TurretMovement {
                entity: entity,
                x: point.x,
                z: point.z,
            });
        }
    }
}

fn shoot_mouse_input(
    mut shoot_event_writer: MessageWriter<Shoot>,
    input: Res<ButtonInput<MouseButton>>,
    players: Query<(Entity, &Player)>,
) {
    if input.just_pressed(MouseButton::Left) {
        let player_entities = players
            .iter()
            .filter(|(_, player)| **player == Player::User);

        for (entity, _) in player_entities {
            shoot_event_writer.write(Shoot { entity });
        }
    }
}
