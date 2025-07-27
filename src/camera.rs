use bevy::{
    app::{App, Startup},
    core_pipeline::core_3d::Camera3d,
    ecs::{component::Component, system::Commands},
    math::Vec3,
    transform::components::Transform,
};

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
        // .add_systems(Startup, spawn_test_cameras);
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

// fn spawn_test_cameras(mut commands: Commands) {
//     commands.spawn((
//         Camera3d {
//             ..Default::default()
//         },
//         Transform::from_xyz(10., 1., 0.).looking_at(Vec3::ZERO, Vec3::Y),
//     ));
// }
