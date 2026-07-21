use avian3d::prelude::{Collider, CollisionEventsEnabled, RigidBody, Sensor};
use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    time::{Time, Timer, TimerMode},
};

use crate::systems::despawn_entity::DespawnEntity;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, despawn_bullet);
}

#[derive(Component)]
#[require(RigidBody::Kinematic, Collider, Sensor, CollisionEventsEnabled)]
pub struct Bullet {
    pub damage: u8,
    despawn_timer: Timer,
}

impl Bullet {
    pub fn new(damage: u8) -> Self {
        Self {
            damage,
            despawn_timer: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}

fn despawn_bullet(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullets.iter_mut() {
        bullet.despawn_timer.tick(time.delta());

        if bullet.despawn_timer.just_finished() {
            commands.entity(entity).insert(DespawnEntity);
        }
    }
}
