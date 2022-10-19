use bevy::prelude::*;
use bevy_egui::{egui::CentralPanel, EguiContext};

use crate::{
    style::register_stylesheet,
    types::{CursorDirection, CustomFontData},
    ActionTrait, ScreenTrait, SettingsState,
};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut writer: EventWriter<CursorDirection>,
) {
    use CursorDirection::*;
    if keyboard_input.just_pressed(KeyCode::Down) {
        writer.send(Down);
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        writer.send(Up);
    } else if keyboard_input.just_pressed(KeyCode::Return) {
        writer.send(Select);
    } else if keyboard_input.just_pressed(KeyCode::Back) {
        writer.send(Back);
    }

    // FIXME: Gamepad
}

pub fn setup_menu_system<State, A, S>(
    mut egui_context: ResMut<EguiContext>,
    settings_state: Res<SettingsState<State, A, S>>,
    mut custom_font: Option<ResMut<CustomFontData>>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    let optional_custom_font = custom_font.as_deref_mut().and_then(|e| e.0.take());
    register_stylesheet(
        &settings_state.menu.stylesheet,
        egui_context.ctx_mut(),
        optional_custom_font,
    );
}

pub fn ui_settings_system<State, A, S>(
    mut egui_context: ResMut<EguiContext>,
    mut settings_state: ResMut<SettingsState<State, A, S>>,
    mut event_writer: EventWriter<A::Event>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        settings_state.menu.show(ui, &mut event_writer);
    });
}

pub fn input_system<State, A, S>(
    mut reader: EventReader<CursorDirection>,
    mut settings_state: ResMut<SettingsState<State, A, S>>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    if let Some(event) = reader.iter().next() {
        settings_state.menu.next(*event)
    }
}
