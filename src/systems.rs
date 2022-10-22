use bevy::prelude::*;
use bevy_egui::{egui::CentralPanel, EguiContext};

use crate::{
    style::{register_stylesheet, Stylesheet},
    types::{CursorDirection, CustomFontData},
    ActionTrait, ScreenTrait, SettingsState,
};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut writer: EventWriter<CursorDirection>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
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

    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
            writer.send(Down);
        } else if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp))
        {
            writer.send(Up);
        } else if button_inputs
            .just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight))
        {
            writer.send(Back);
        } else if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            writer.send(Select);
        } else if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::East)) {
            writer.send(Back);
        }

        if axes.is_changed() {
            for (axis, check_negative, action) in [
                (GamepadAxisType::LeftStickX, true, Back),
                (GamepadAxisType::LeftStickY, true, Down),
                (GamepadAxisType::LeftStickY, false, Up),
                (GamepadAxisType::RightStickX, true, Back),
                (GamepadAxisType::RightStickY, true, Down),
                (GamepadAxisType::RightStickY, false, Up),
            ] {
                if let Some(value) = axes.get(GamepadAxis::new(gamepad, axis)) {
                    if (check_negative && value < -0.1) || (!check_negative && value > 0.1) {
                        writer.send(action);
                    }
                }
            }
        }
    }
}

pub fn setup_menu_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut custom_font: Option<ResMut<CustomFontData>>,
    stylesheet: Option<Res<Stylesheet>>,
) {
    let valid_stylesheet = stylesheet.map(|e| e.clone()).unwrap_or_default();
    let optional_custom_font = custom_font.as_deref_mut().and_then(|e| e.0.take());
    register_stylesheet(
        &valid_stylesheet,
        egui_context.ctx_mut(),
        optional_custom_font,
    );
    // insert again, might override the old one with itself
    commands.insert_resource(valid_stylesheet);
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

// pub fn update_gamepads_system(
//     gamepads: Res<Gamepads>,
//     button_inputs: Res<Input<GamepadButton>>,
//     button_axes: Res<Axis<GamepadButton>>,
//     axes: Res<Axis<GamepadAxis>>,
// ) {
//     for gamepad in gamepads.iter().cloned() {
//         if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
//             info!("{:?} just pressed South", gamepad);
//         } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
//         {
//             info!("{:?} just released South", gamepad);
//         }

//         let right_trigger = button_axes
//             .get(GamepadButton::new(
//                 gamepad,
//                 GamepadButtonType::RightTrigger2,
//             ))
//             .unwrap();
//         if right_trigger.abs() > 0.01 {
//             info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
//         }

//         let left_stick_x = axes
//             .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
//             .unwrap();
//         if left_stick_x.abs() > 0.01 {
//             info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
//         }
//     }
// }
