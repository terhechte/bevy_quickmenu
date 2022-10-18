use bevy::prelude::*;

mod menu;
mod settings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(settings::SettingsPlugin)
        .run();
}
