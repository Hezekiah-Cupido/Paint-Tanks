pub(crate) mod basic_tank_body;

use avian3d::prelude::{AngularVelocity, LinearVelocity};
use bevy::{
    app::{App, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        message::{Message, MessageReader},
        query::With,
        system::{Commands, EntityCommands, Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

pub fn plugin(app: &mut App) {
    app.add_message::<Movement>().add_systems(Update, move_tank);
}

const LINEAR_MOVEMENT_SPEED: f32 = 10.;
const ANGULAR_MOVEMENT_SPEED: f32 = 50.;

pub trait TankBodySpawner {
    fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        asset_server: &AssetServer,
    ) -> EntityCommands<'a>;
}

#[derive(Message)]
pub struct Movement {
    pub entity: Entity,
    pub movement_type: MovementType,
}

pub enum MovementType {
    Linear(i8),
    Angular(i8),
}

#[derive(Component, Default)]
#[require(Transform::from_xyz(0., 0.5, 0.))]
pub struct TankBody;

fn move_tank(
    mut movement_event_reader: MessageReader<Movement>,
    mut tanks: Query<(&mut LinearVelocity, &mut AngularVelocity, &Transform), With<TankBody>>,
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
