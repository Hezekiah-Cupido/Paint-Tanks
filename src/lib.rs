use avian3d::{
    PhysicsPlugins,
    prelude::{PhysicsDebugPlugin, PhysicsGizmos},
};
use bevy::{
    DefaultPlugins,
    app::{App, Plugin},
    color::Color,
    gizmos::{AppGizmoBuilder, config::GizmoConfig},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use entities::lights;

use crate::entities::despawn_entity;

mod camera;
mod diagnostics;
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
            diagnostics::plugin,
        ))
        .add_plugins((
            camera::plugin,
            despawn_entity::plugin,
            lights::plugin,
            maps::plugin,
            tank::plugin,
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..Default::default()
            },
            GizmoConfig::default(),
        );
    }
}
