use bevy::prelude::*;
// use bevy_egui::{egui::CentralPanel, EguiContext};

use crate::{
    style::Stylesheet,
    types::{self, ButtonComponent, CustomFontData, NavigationEvent, QuickMenuComponent},
    ActionTrait, RedrawEvent, ScreenTrait, Selections, SettingsState,
};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut writer: EventWriter<NavigationEvent>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    use NavigationEvent::*;
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
    // mut egui_context: ResMut<EguiContext>,
    mut custom_font: Option<ResMut<CustomFontData>>,
    stylesheet: Option<Res<Stylesheet>>,
    mut redraw_writer: EventWriter<RedrawEvent>,
) {
    commands.insert_resource(Selections::default());
    let valid_stylesheet = stylesheet.map(|e| e.clone()).unwrap_or_default();
    let optional_custom_font = custom_font.as_deref_mut().and_then(|e| e.0.take());
    // FIXME: Fonts
    // register_stylesheet(
    //     &valid_stylesheet,
    //     egui_context.ctx_mut(),
    //     optional_custom_font,
    // );
    // insert again, might override the old one with itself
    commands.insert_resource(valid_stylesheet);
    redraw_writer.send(RedrawEvent);
}

pub fn ui_settings_system<State, A, S>(
    settings_state: Res<SettingsState<State, A, S>>,
    mut redraw_writer: EventWriter<RedrawEvent>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    // if settings_state.is_changed() {
    //     println!("Settings changed");
    //     redraw_writer.send(RedrawEvent);
    // }
}

pub fn redraw_system<State, A, S>(
    mut commands: Commands,
    existing: Query<Entity, With<QuickMenuComponent>>,
    mut settings_state: ResMut<SettingsState<State, A, S>>,
    // mut event_writer: EventWriter<A::Event>,
    selections: Res<Selections>,
    mut redraw_reader: EventReader<RedrawEvent>,
    //  mut event_writer: EventWriter<RedrawEvent>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    for _ in redraw_reader.iter() {
        println!("redraw");
        for item in existing.iter() {
            commands.entity(item).despawn_recursive();
        }
        settings_state.menu.show(&selections, &mut commands);
        // if !redraw {
        //     redraw_writer.send(RedrawEvent)
        // }
    }
}

pub fn input_system<State, A, S>(
    mut reader: EventReader<NavigationEvent>,
    mut settings_state: ResMut<SettingsState<State, A, S>>,
    mut redraw_writer: EventWriter<RedrawEvent>,
    mut selections: ResMut<Selections>,
    mut event_writer: EventWriter<A::Event>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    if let Some(event) = reader.iter().next() {
        if let Some(selection) = settings_state.menu.apply_event(event, &mut selections) {
            settings_state
                .menu
                .handle_selection(&selection, &mut event_writer);
        }
        redraw_writer.send(RedrawEvent);
    }
}

pub fn mouse_system<State, A, S>(
    mut reader: EventReader<NavigationEvent>,
    mut settings_state: ResMut<SettingsState<State, A, S>>,
    mut interaction_query: Query<
        (&Interaction, &types::ButtonComponent<State, A, S>),
        Changed<Interaction>,
    >,
    mut event_writer: EventWriter<A::Event>,
    mut selections: ResMut<Selections>,
) where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    for (
        interaction,
        ButtonComponent {
            style,
            selection,
            menu_identifier,
        },
    ) in &mut interaction_query
    {
        match *interaction {
            Interaction::Clicked => {
                settings_state
                    .menu
                    .handle_selection(&selection, &mut event_writer);
                selections.0.insert(menu_identifier.0, menu_identifier.1);
                println!("SELECTED");
            }
            _ => (),
        }
    }
}
