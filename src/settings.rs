use bevy::prelude::*;
use bevy_egui::*;

use crate::menu;

#[derive(Debug)]
struct CustomState {}

#[derive(Resource)]
struct SettingsState {
    menu: menu::NavigationMenu<CustomState>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingsState {
            menu: menu::NavigationMenu::new(CustomState {}),
        })
        .add_plugin(EguiPlugin)
        .add_startup_system(setup_settings)
        .add_system_set(
            SystemSet::new()
                .with_system(ui_settings_system)
                .with_system(keyboard_input_system),
        );
    }
}

fn setup_settings(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    //?
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut settings_state: ResMut<SettingsState>,
) {
    use menu::CursorDirection::*;
    if keyboard_input.just_pressed(KeyCode::Down) {
        settings_state.menu.next(Down)
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        settings_state.menu.next(Up)
    } else if keyboard_input.just_pressed(KeyCode::Return) {
        settings_state.menu.next(Select)
    } else if keyboard_input.just_pressed(KeyCode::Back) {
        settings_state.menu.next(Back)
    }

    // FIXME: Gamepad
}

fn ui_settings_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut settings_state: ResMut<SettingsState>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.add(&mut settings_state.menu)
    });
}
