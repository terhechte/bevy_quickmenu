use bevy::{prelude::*, utils::HashMap};

use bevy_quickmenu::{
    helpers::ControlDevice, style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem,
    QuickMenuPlugin, ScreenTrait, SettingsState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SettingsPlugin)
        .run();
}

#[derive(Debug)]
enum MyEvent {
    CloseSettings,
}

#[derive(Debug, Clone)]
struct CustomState {
    sound_on: bool,
    gamepads: Vec<Gamepad>,
    controls: HashMap<usize, ControlDevice>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register a event that can be called from your action handler
            .add_event::<MyEvent>()
            // The plugin
            .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::default())
            // Some systems
            .add_startup_system(setup)
            .add_system(event_reader)
            .add_system(update_gamepads_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle::default());
    let font = asset_server.load("font.ttf");
    // Create a default stylesheet. You can customize these as you wish
    let sheet = Stylesheet::with_font(font);

    // The settings state that will be handed to menus, screens and actions.
    // If you remove this resource, the menu will disappear
    commands.insert_resource(SettingsState::new(
        CustomState {
            sound_on: true,
            gamepads: Vec::new(),
            controls: [
                (0, ControlDevice::keyboard1()),
                (1, ControlDevice::keyboard2()),
                (2, ControlDevice::keyboard3()),
                (3, ControlDevice::keyboard4()),
            ]
            .into(),
        },
        Screens::Root,
        Some(sheet),
    ))
}

fn update_gamepads_system(
    gamepads: Res<Gamepads>,
    mut settings_state: ResMut<SettingsState<CustomState, Actions, Screens>>,
) {
    let gamepads = gamepads.iter().collect();
    if settings_state.state().gamepads != gamepads {
        settings_state.state_mut().gamepads = gamepads;
    }
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
    type Event = MyEvent;
    fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
        println!("Handle event");
        match self {
            Actions::Close => event_writer.send(MyEvent::CloseSettings),
            Actions::SoundOn => state.sound_on = true,
            Actions::SoundOff => state.sound_on = false,
            Actions::Control(p, d) => {
                state.controls.insert(*p, *d);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Controls,
    Sound,
    Soundx,
    Player(usize),
}

impl ScreenTrait for Screens {
    type Action = Actions;
    fn resolve(&self, state: &CustomState) -> Menu<Actions, Screens, CustomState> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Controls => controls_menu(state),
            Screens::Sound => sound_menu(state),
            Screens::Soundx => sound_menu(state),
            Screens::Player(p) => player_controls_menu(state, *p),
        }
    }
}

fn root_menu(_state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu {
        id: "root",
        entries: vec![
            MenuItem::headline("Settings"),
            MenuItem::action("Back", Actions::Close).with_icon(MenuIcon::Back),
            MenuItem::screen("Sound", Screens::Sound).with_icon(MenuIcon::Sound),
            MenuItem::headline("Settings"),
            MenuItem::screen("Soundx", Screens::Soundx).with_icon(MenuIcon::Sound),
            MenuItem::screen("Controls", Screens::Controls).with_icon(MenuIcon::Controls),
            MenuItem::headline("Settings"),
        ],
    }
}

fn sound_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu {
        id: "sound",
        entries: vec![
            MenuItem::label("Toggles sound and music"),
            MenuItem::action("On", Actions::SoundOn).checked(state.sound_on),
            MenuItem::action("Off", Actions::SoundOff).checked(!state.sound_on),
        ],
    }
}

fn controls_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    let mut players: Vec<usize> = state.controls.keys().copied().collect();
    players.sort();
    Menu {
        id: "controls",
        entries: players
            .into_iter()
            .map(|player| MenuItem::screen(format!("Player {player}"), Screens::Player(player)))
            .collect(),
    }
}

fn player_controls_menu(state: &CustomState, player: usize) -> Menu<Actions, Screens, CustomState> {
    let selected_control = state.controls[&player];
    let mut entries = vec![
        ControlDevice::keyboard1(),
        ControlDevice::keyboard2(),
        ControlDevice::keyboard3(),
        ControlDevice::keyboard4(),
    ];
    entries.append(
        &mut state
            .gamepads
            .iter()
            .map(|e| ControlDevice::Gamepad { gamepad_id: e.id })
            .collect(),
    );
    let entries = entries
        .into_iter()
        .map(|entry| {
            MenuItem::action(entry.to_string(), Actions::Control(player, entry))
                .checked(entry.id() == selected_control.id())
        })
        .collect();
    Menu {
        id: "players",
        entries,
    }
}

fn event_reader(mut event_reader: EventReader<MyEvent>) {
    for event in event_reader.iter() {
        dbg!(event);
    }
}
