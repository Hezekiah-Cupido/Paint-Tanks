use bevy::{
    app::{App, Startup},
    ecs::{component::Component, system::Commands},
    math::Vec3,
    pbr::SpotLight,
    transform::components::Transform,
};

#[derive(Component)]
pub struct Light;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_light);
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Light,
        SpotLight {
            ..Default::default()
        },
        Transform::from_xyz(0., 15., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
