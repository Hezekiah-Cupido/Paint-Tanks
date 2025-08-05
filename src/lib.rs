use avian3d::{
    prelude::{PhysicsDebugPlugin, PhysicsGizmos},
    PhysicsPlugins,
};
use bevy::{
    app::{App, Plugin}, color::Color, gizmos::{config::GizmoConfig, AppGizmoBuilder}, DefaultPlugins
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use entities::lights;

mod camera;
mod entities;
mod maps;
mod tank;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_plugins((lights::plugin, camera::plugin, maps::plugin, tank::plugin))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..Default::default()
            },
            GizmoConfig::default(),
        );
    }
}
