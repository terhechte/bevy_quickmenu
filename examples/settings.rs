use bevy::prelude::*;

use bevy_quickmenu::{
    egui::*,
    make_menu,
    style::{Style, Stylesheet},
    ActionTrait, CursorDirection, CustomFontData, Menu, MenuItem, MenuSelection, NavigationMenu,
    QuickMenuPlugin, ScreenTrait, SettingsState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SettingsPlugin)
        .run();
}

const FONT_DATA: &[u8] = include_bytes!("font.ttf");

#[derive(Debug)]
enum MyEvent {
    Test,
}

#[derive(Debug, Clone)]
struct CustomState {}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let sheet = Stylesheet {
            button: Some(Style {
                size: 40.0,
                ..Default::default()
            }),
            ..Default::default()
        };
        app.insert_resource(CustomFontData(Some(FONT_DATA)))
            .insert_resource(SettingsState::new(
                CustomState {},
                Screens::Root,
                Some(sheet),
            ))
            .add_event::<MyEvent>()
            .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::default())
            .add_system(event_reader);
    }
}

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
