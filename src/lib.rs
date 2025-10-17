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

use crate::entities::turret::{self, basic_turret};

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
        .add_plugins((
            lights::plugin,
            camera::plugin,
            maps::plugin,
            turret::plugin,
            basic_turret::plugin,
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
