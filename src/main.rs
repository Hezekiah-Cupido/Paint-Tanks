use bevy::app::App;
use paint_tanks::AppPlugin;

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run();
}
