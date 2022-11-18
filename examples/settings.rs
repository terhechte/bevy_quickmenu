use bevy::{prelude::*, utils::HashMap};

use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuOptions, QuickMenuPlugin,
    ScreenTrait, SettingsState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SettingsPlugin)
        .run();
}

/// This custom event can be emitted by the action handler (below) in order to
/// process actions with access to the bevy ECS
#[derive(Debug)]
enum MyEvent {
    CloseSettings,
}

/// This state represents the UI. Mutations to this state (via `SettingsState::state_mut`)
/// cause a re-render of the menu UI
#[derive(Debug, Clone)]
struct CustomState {
    sound_on: bool,
    gamepads: Vec<(Gamepad, String)>,
    controls: HashMap<usize, ControlDevice>,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register a event that can be called from your action handler
            .add_event::<MyEvent>()
            // The plugin
            .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::new())
            // Some systems
            .add_startup_system(setup)
            .add_system(event_reader)
            .add_system(update_gamepads_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    // Create a default stylesheet. You can customize these as you wish
    let sheet = Stylesheet::default();

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

/// Whenever a new gamepad connects, get the known gamepads and their names
/// into our state
fn update_gamepads_system(
    gamepads: Res<Gamepads>,
    mut settings_state: ResMut<SettingsState<CustomState, Actions, Screens>>,
) {
    let gamepads = gamepads
        .iter()
        .map(|p| {
            (
                p,
                gamepads.name(p).map(|s| s.to_owned()).unwrap_or_default(),
            )
        })
        .collect();
    if settings_state.state().gamepads != gamepads {
        settings_state.state_mut().gamepads = gamepads;
    }
}

/// The possible actions in our settings
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Close,
    SoundOn,
    SoundOff,
    Control(usize, ControlDevice),
}

/// Handle the possible actions
impl ActionTrait for Actions {
    type State = CustomState;
    type Event = MyEvent;
    fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
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

/// All possible screens in our settings
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Controls,
    Sound,
    Player(usize),
}

impl ScreenTrait for Screens {
    type Action = Actions;
    fn resolve(&self, state: &CustomState) -> Menu<Actions, Screens, CustomState> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Controls => controls_menu(state),
            Screens::Sound => sound_menu(state),
            Screens::Player(p) => player_controls_menu(state, *p),
        }
    }
}

/// The `root` menu that is displayed first
fn root_menu(_state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu::new(
        "root",
        vec![
            MenuItem::headline("Settings"),
            MenuItem::action("Back", Actions::Close).with_icon(MenuIcon::Back),
            MenuItem::screen("Sound", Screens::Sound).with_icon(MenuIcon::Sound),
            MenuItem::screen("Controls", Screens::Controls).with_icon(MenuIcon::Controls),
        ],
    )
}

/// This is displayed if the user selects `Sound` in the `root_menu`
fn sound_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    Menu::new(
        "sound",
        vec![
            MenuItem::label("Toggles sound and music"),
            MenuItem::action("On", Actions::SoundOn).checked(state.sound_on),
            MenuItem::action("Off", Actions::SoundOff).checked(!state.sound_on),
        ],
    )
}

/// This is displayed if the user selects `Controls` in the `root_menu`
fn controls_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
    let mut players: Vec<usize> = state.controls.keys().copied().collect();
    players.sort();
    Menu::new(
        "controls",
        players
            .into_iter()
            .map(|player| MenuItem::screen(format!("Player {player}"), Screens::Player(player)))
            .collect(),
    )
}

/// This is displayed if the user selects a player in the `controls_menu`
fn player_controls_menu(state: &CustomState, player: usize) -> Menu<Actions, Screens, CustomState> {
    let selected_control = state.controls[&player];
    // Get the Keyboards
    let mut entries: Vec<_> = vec![
        ControlDevice::keyboard1(),
        ControlDevice::keyboard2(),
        ControlDevice::keyboard3(),
        ControlDevice::keyboard4(),
    ]
    .iter()
    .map(|kb| {
        MenuItem::action(kb.to_string(), Actions::Control(player, *kb))
            .checked(kb.id() == selected_control.id())
    })
    .collect();

    // Get the GamePads
    for (pad, title) in &state.gamepads {
        let device = ControlDevice::Gamepad { gamepad_id: pad.id };
        entries.push(
            MenuItem::action(title, Actions::Control(player, device))
                .checked(device.id() == selected_control.id()),
        )
    }

    Menu::new("players", entries)
}

/// This allows to react to actions with custom bevy resources or eventwriters or queries.
/// In this example we use it to close the menu
fn event_reader(mut commands: Commands, mut event_reader: EventReader<MyEvent>) {
    for event in event_reader.iter() {
        match event {
            MyEvent::CloseSettings => bevy_quickmenu::cleanup(&mut commands),
        }
    }
}

// Abstractions over control devices

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ControlDevice {
    Gamepad {
        gamepad_id: usize,
    },
    Keyboard {
        title: &'static str,
        description: &'static str,
        keyboard_id: usize,
        left: KeyCode,
        right: KeyCode,
        action: KeyCode,
    },
}

impl ControlDevice {
    pub fn id(&self) -> usize {
        match self {
            ControlDevice::Gamepad { gamepad_id, .. } => *gamepad_id,
            ControlDevice::Keyboard { keyboard_id, .. } => *keyboard_id,
        }
    }
}

impl std::fmt::Display for ControlDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlDevice::Gamepad { gamepad_id } => {
                f.write_fmt(format_args!("Gamepad {gamepad_id}",))
            }
            ControlDevice::Keyboard { title, .. } => f.write_fmt(format_args!("{title}")),
        }
    }
}

impl ControlDevice {
    pub fn keyboard1() -> ControlDevice {
        ControlDevice::Keyboard {
            title: "Keyboard 1",
            description: "Left / Right + M",
            keyboard_id: 42001,
            left: KeyCode::Left,
            right: KeyCode::Right,
            action: KeyCode::M,
        }
    }

    pub fn keyboard2() -> ControlDevice {
        ControlDevice::Keyboard {
            title: "Keyboard 2",
            description: "A / D + B",
            keyboard_id: 42002,
            left: KeyCode::A,
            right: KeyCode::D,
            action: KeyCode::B,
        }
    }
    pub fn keyboard3() -> ControlDevice {
        ControlDevice::Keyboard {
            title: "Keyboard 3",
            description: "I / O + K",
            keyboard_id: 42003,
            left: KeyCode::I,
            right: KeyCode::O,
            action: KeyCode::K,
        }
    }
    pub fn keyboard4() -> ControlDevice {
        ControlDevice::Keyboard {
            title: "Keyboard 4",
            description: "T / Y + H",
            keyboard_id: 42004,
            left: KeyCode::T,
            right: KeyCode::Y,
            action: KeyCode::H,
        }
    }
}
