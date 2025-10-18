use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        hierarchy::Children,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
        world::World,
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{Vec3, primitives::InfinitePlane3d},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::{
    camera::MainCamera,
    entities::{
        tank_body::{Movement, MovementType, TankBodySpawner, basic_tank_body::BasicTankBody},
        turret::{Shoot, Turret, TurretMovement, TurretSpawner, basic_turret::BasicTurret},
    },
};

#[derive(Resource)]
pub struct TankAssets {
    turret: Box<dyn TurretSpawner + Send + Sync>,
    tank_body: Box<dyn TankBodySpawner + Send + Sync>,
}

#[derive(Component)]
pub struct Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (load_tank_assets, spawn_tank).chain())
        .add_systems(
            Update,
            (keyboard_input, mouse_input, mouse_button_input),
        );
}

fn load_tank_assets(mut commands: Commands) {
    commands.insert_resource(TankAssets {
        turret: Box::new(BasicTurret {}),
        tank_body: Box::new(BasicTankBody {}),
    });
}

fn spawn_tank(mut commands: Commands, world: &World, tank_assets: Res<TankAssets>) {
    tank_assets
        .tank_body
        .spawn(&mut commands, world)
        .insert(Player)
        .with_children(|parent| {
            tank_assets.turret.spawn_turret(parent, world);
        });
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
