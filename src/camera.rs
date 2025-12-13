use bevy::{
    app::{App, Startup},
    camera::Camera3d,
    ecs::{component::Component, system::Commands},
    math::Vec3,
    transform::components::Transform,
};

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d {
            ..Default::default()
        },
        Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}
