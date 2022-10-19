use bevy::prelude::*;

use bevy_quickmenu::{
    egui::*, make_menu, ActionTrait, CursorDirection, Menu, MenuItem, MenuSelection,
    NavigationMenu, QuickMenuPlugin, ScreenTrait, SettingsState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SettingsPlugin)
        .run();
}

#[derive(Debug)]
enum MyEvent {
    Test,
}

#[derive(Debug, Clone)]
struct CustomState {}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingsState::new(CustomState {}, Screens::Root))
            .add_event::<MyEvent>()
            .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::default())
            .add_system(event_reader);
    }
}

// fn input_system(
//     mut reader: EventReader<CursorDirection>,
//     mut settings_state: ResMut<SettingsState>,
// ) {
//     if let Some(event) = reader.iter().next() {
//         settings_state.menu.next(*event)
//     }
// }

// fn ui_settings_system(
//     mut commands: Commands,
//     mut egui_context: ResMut<EguiContext>,
//     mut settings_state: ResMut<SettingsState>,
//     mut event_writer: EventWriter<MyEvent>,
// ) {
//     egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
//         settings_state.menu.show(ui, &mut event_writer);
//     });
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Close,
    SoundOn,
    SoundOff,
    // Control(usize, ControlDevice),
}

impl ActionTrait for Actions {
    type State = CustomState;
    type Event = MyEvent;
    fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
        event_writer.send(MyEvent::Test);
        match self {
            Actions::Close => return,
            Actions::SoundOn => return,
            Actions::SoundOff => return,
            // Actions::Control(_, _) => return,
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
    fn resolve(&self, state: &mut CustomState) -> Menu<Actions, Screens, CustomState> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Players => sound_menu(state),
            Screens::Controls => sound_menu(state),
            Screens::Sound => sound_menu(state),
            Screens::Player(_) => sound_menu(state),
        }
    }
}

// fn a_screen(
//     menu: &mut NavigationMenu<CustomState, Actions, Screens>
// ) -> Option<MenuSelection<Actions, Screens, CustomState>> {
//     // need a way to wrap the return here without exposing ui
//     /// ???
// }

fn root_menu(_state: &mut CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu {
        id: Id::new("root"),
        entries: vec![
            MenuItem::action("Back", Actions::Close),
            MenuItem::screen("Sound", Screens::Sound),
            MenuItem::screen("Controls", Screens::Controls),
        ],
    }
}

fn sound_menu(_state: &mut CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu {
        id: Id::new("sound"),
        entries: vec![
            MenuItem::action("On", Actions::SoundOn),
            MenuItem::action("Off", Actions::SoundOff),
        ],
    }
}

fn event_reader(mut event_reader: EventReader<MyEvent>) {
    for event in event_reader.iter() {
        dbg!(event);
    }
}
