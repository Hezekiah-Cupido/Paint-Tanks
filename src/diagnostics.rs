use bevy::{
    app::App,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    render::diagnostic::RenderDiagnosticsPlugin,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        SystemInformationDiagnosticsPlugin::default(),
        RenderDiagnosticsPlugin::default(),
    ));
}
