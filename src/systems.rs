use bevy::prelude::*;

use crate::{
    types::{self, ButtonComponent, CleanUpUI, MenuAssets, NavigationEvent, QuickMenuComponent},
    ActionTrait, MenuState, RedrawEvent, ScreenTrait, Selections,
};

pub fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<NavigationEvent>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    use NavigationEvent::*;
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        writer.send(Down);
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        writer.send(Up);
    } else if keyboard_input.just_pressed(KeyCode::Enter) {
        writer.send(Select);
    } else if keyboard_input.just_pressed(KeyCode::Backspace) {
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
            || button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West))
        {
            writer.send(Select);
        } else if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::East))
            || button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North))
        {
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

pub fn redraw_system<S>(
    mut commands: Commands,
    existing: Query<Entity, With<QuickMenuComponent>>,
    mut menu_state: ResMut<MenuState<S>>,
    selections: Res<Selections>,
    redraw_reader: EventReader<RedrawEvent>,
    assets: Res<MenuAssets>,
    // mut initial_render_done: Local<bool>,
) where
    S: ScreenTrait + 'static,
{
    let mut can_redraw = !redraw_reader.is_empty();
    if !menu_state.initial_render_done {
        menu_state.initial_render_done = true;
        can_redraw = true;
    }
    if can_redraw {
        for item in existing.iter() {
            commands.entity(item).despawn_recursive();
        }
        menu_state.menu.show(&assets, &selections, &mut commands);
    }
}

pub fn input_system<S>(
    mut reader: EventReader<NavigationEvent>,
    mut menu_state: ResMut<MenuState<S>>,
    mut redraw_writer: EventWriter<RedrawEvent>,
    mut selections: ResMut<Selections>,
    mut event_writer: EventWriter<<<S as ScreenTrait>::Action as ActionTrait>::Event>,
) where
    S: ScreenTrait + 'static,
{
    if let Some(event) = reader.read().next() {
        if let Some(selection) = menu_state.menu.apply_event(event, &mut selections) {
            menu_state
                .menu
                .handle_selection(&selection, &mut event_writer);
        }
        redraw_writer.send(RedrawEvent);
    }
}

#[allow(clippy::type_complexity)]
pub fn mouse_system<S>(
    mut menu_state: ResMut<MenuState<S>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &types::ButtonComponent<S>,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    mut event_writer: EventWriter<<<S as ScreenTrait>::Action as ActionTrait>::Event>,
    mut selections: ResMut<Selections>,
    mut redraw_writer: EventWriter<RedrawEvent>,
) where
    S: ScreenTrait + 'static,
{
    for (
        interaction,
        ButtonComponent {
            selection,
            style,
            menu_identifier,
            selected,
        },
        mut background_color,
    ) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                // pop to the chosen selection stack entry
                menu_state.menu.pop_to_selection(selection);

                // pre-select the correct row
                selections
                    .0
                    .insert(menu_identifier.0.clone(), menu_identifier.1);
                if let Some(current) = menu_state
                    .menu
                    .apply_event(&NavigationEvent::Select, &mut selections)
                {
                    menu_state
                        .menu
                        .handle_selection(&current, &mut event_writer);
                    redraw_writer.send(RedrawEvent);
                }
            }
            Interaction::Hovered => {
                if !selected {
                    background_color.0 = style.hover.bg;
                }
            }
            Interaction::None => {
                if !selected {
                    background_color.0 = style.normal.bg;
                }
            }
        }
    }
}

/// If the `CleanUpUI` `Resource` is available, remove the menu and then the resource.
/// This is used to close the menu when it is not needed anymore.
pub fn cleanup_system<S>(
    mut commands: Commands,
    existing: Query<Entity, With<types::QuickMenuComponent>>,
) where
    S: ScreenTrait + 'static,
{
    // Remove all menu elements
    for item in existing.iter() {
        commands.entity(item).despawn_recursive();
    }
    // Remove the resource again
    commands.remove_resource::<CleanUpUI>();
    // Remove the state
    commands.remove_resource::<MenuState<S>>();
}
