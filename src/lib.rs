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
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin, bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};
use bevy_skein::SkeinPlugin;
use entities::lights;

use crate::{
    entities::paintable_surface::{self, PaintableSurfacePlugin},
    game_state::GameStatePlugin,
    maps::MapPlugin,
    systems::despawn_entity,
};

mod camera;
mod diagnostics;
mod entities;
mod game_state;
mod maps;
mod systems;
mod tank;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            DefaultInspectorConfigPlugin,
            EguiPlugin::default(),
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default(),
            SkeinPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins((
            GameStatePlugin,
            MapPlugin,
            PaintableSurfacePlugin,
            camera::plugin,
            despawn_entity::plugin,
            // diagnostics::plugin,
            lights::plugin,
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
