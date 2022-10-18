use bevy::prelude::*;
use bevy_egui::{
    egui::{self, *},
    *,
};

use crate::menu::{
    make_menu, ActionTrait, CursorDirection, MenuItem, MenuSelection, NavigationMenu, ScreenTrait,
};

#[derive(Debug, Clone)]
struct CustomState {}

#[derive(Resource)]
struct SettingsState {
    menu: NavigationMenu<CustomState, Actions, Screens>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingsState {
            menu: NavigationMenu::new(CustomState {}, Screens::Root),
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
    use CursorDirection::*;
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

// Menus

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ControlDevice {
    Keyboard(usize),
    Gamepad(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Close,
    SoundOn,
    SoundOff,
    Control(usize, ControlDevice),
}

impl ActionTrait for Actions {
    type State = CustomState;
    fn handle(&self, state: &mut CustomState) {
        match self {
            Actions::Close => return,
            Actions::SoundOn => return,
            Actions::SoundOff => return,
            Actions::Control(_, _) => return,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Players,
    Controls,
    Sound,
    Player(usize),
}

impl ScreenTrait for Screens {
    type Action = Actions;
    fn resolve(
        &self,
        ui: &mut Ui,
        state: &mut CustomState,
        cursor_direction: Option<CursorDirection>,
    ) -> Option<MenuSelection<Actions, Screens, CustomState>> {
        match self {
            Screens::Root => root_menu(ui, cursor_direction),
            Screens::Players => sound_menu(ui, cursor_direction),
            Screens::Controls => sound_menu(ui, cursor_direction),
            Screens::Sound => sound_menu(ui, cursor_direction),
            Screens::Player(_) => sound_menu(ui, cursor_direction),
        }
    }
}

fn root_menu(
    ui: &mut Ui,
    cursor_direction: Option<CursorDirection>,
) -> Option<MenuSelection<Actions, Screens, CustomState>> {
    make_menu(
        ui,
        Id::new("root"),
        cursor_direction,
        vec![
            MenuItem::action("Back", Actions::Close),
            MenuItem::screen("Sound", Screens::Sound),
            MenuItem::screen("Controls", Screens::Controls),
        ],
    )
}

fn sound_menu(
    ui: &mut Ui,
    cursor_direction: Option<CursorDirection>,
) -> Option<MenuSelection<Actions, Screens, CustomState>> {
    make_menu(
        ui,
        Id::new("sound"),
        cursor_direction,
        vec![
            MenuItem::action("On", Actions::SoundOn),
            MenuItem::action("Off", Actions::SoundOff),
        ],
    )
}
