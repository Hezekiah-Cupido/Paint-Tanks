pub(crate) mod basic_turret;

use avian3d::math::PI;
use bevy::{
    app::{App, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        hierarchy::ChildOf,
        query::With,
        relationship::RelatedSpawnerCommands,
        system::{Query, Res},
    },
    math::{Vec3, Vec3Swizzles, ops::acos},
    transform::components::{GlobalTransform, Transform},
};

const TURRET_ROTATION_SPEED: f32 = 3.;

pub trait TurretSpawner {
    fn spawn_turret(
        &self,
        commands: &mut RelatedSpawnerCommands<'_, ChildOf>,
        asset_server: &AssetServer,
    );
}

#[derive(Component)]
pub struct BulletSpawner;

#[derive(Event)]
pub struct Shoot {
    pub turret: Entity,
}

#[derive(Event)]
pub struct TurretMovement {
    pub turret_entity: Entity,
    pub x: f32,
    pub z: f32,
}

pub fn plugin(app: &mut App) {
    app.add_event::<TurretMovement>()
        .add_event::<Shoot>()
        .add_systems(Update, move_turret);
}

#[derive(Component, Default)]
pub struct Turret;

fn move_turret(
    mut turret_movement_event_reader: EventReader<TurretMovement>,
    mut turret_transforms: Query<(&mut Transform, &GlobalTransform), With<Turret>>,
    time: Res<bevy::time::Time>,
) {
    for event in turret_movement_event_reader.read() {
        if let Ok((mut turret_transform, turret_global_transform)) =
            turret_transforms.get_mut(event.turret_entity)
        {
            let turret_translation = turret_transform.translation.clone();

            let x = event.x;
            let y = turret_translation.y;
            let z = event.z;

            let to_cursor = (Vec3::new(x, y, z) - turret_translation).normalize();

            let _turret_rotation = turret_transform.rotation.clone();

            let turret_rotation_x = (turret_global_transform.rotation() * Vec3::X).normalize();
            let turret_rotation_y = turret_global_transform.forward().normalize();

            let rotation_angle = acos(turret_rotation_y.xz().dot(to_cursor.xz()).clamp(-1., 1.));

            if rotation_angle - (PI / 180.) > f32::EPSILON {
                let rotation_sign = -f32::copysign(1., turret_rotation_x.dot(to_cursor));

                let turret_rotation_rate: f32 =
                    (TURRET_ROTATION_SPEED / rotation_angle).clamp(1., TURRET_ROTATION_SPEED);
                turret_transform.rotate_y(
                    rotation_sign * rotation_angle * turret_rotation_rate * time.delta_secs(),
                );
            }
        }
    }
}
