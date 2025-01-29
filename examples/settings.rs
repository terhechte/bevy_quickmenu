// //! A Settings example
// //! This example mimics in-game settings with player and sound controls.
// //! It also adds some types to simulate gamepad and keyboard settings.
// //! It also shows how to get from the Settings to the game and back.
// //! Due to the way Bevy handles GameStates (which will soon be rewritten),
// //! composing menus and games looks a bit convoluted.
// use bevy::{prelude::*, utils::HashMap};

// use bevy_quickmenu::{
//     style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuState, QuickMenuPlugin,
//     ScreenTrait,
// };

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
// enum GameState {
//     #[default]
//     Settings,
//     Game,
// }

// impl GameState {
//     fn is_game(state: Res<State<GameState>>) -> bool {
//         state.get() == &GameState::Game
//     }

//     fn is_settings(state: Res<State<GameState>>) -> bool {
//         state.get() == &GameState::Settings
//     }
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .init_state::<GameState>()
//         .add_plugins(settings::SettingsPlugin)
//         .add_plugins(game::Game)
//         .run();
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera3d::default());
// }

// mod settings {
//     use bevy::input::gamepad::GamepadAxisChangedEvent;

//     use super::*;

//     /// This custom event can be emitted by the action handler (below) in order to
//     /// process actions with access to the bevy ECS
//     #[derive(Debug, Event)]
//     enum MyEvent {
//         CloseSettings,
//     }

//     /// This state represents the UI. Mutations to this state (via `MenuState::state_mut`)
//     /// cause a re-render of the menu UI
//     #[derive(Debug, Clone)]
//     struct CustomState {
//         sound_on: bool,
//         gamepads: Vec<(usize, String)>,
//         controls: HashMap<usize, ControlDevice>,
//         logo: Handle<Image>,
//     }

//     pub struct SettingsPlugin;

//     impl Plugin for SettingsPlugin {
//         fn build(&self, app: &mut App) {
//             app
//                 // Register a event that can be called from your action handler
//                 .add_event::<MyEvent>()
//                 // The plugin
//                 .add_plugins(QuickMenuPlugin::<Screens>::new())
//                 // Some systems
//                 .add_systems(OnEnter(GameState::Settings), setup_system)
//                 .add_systems(
//                     Update,
//                     (event_reader, update_gamepads_system).run_if(GameState::is_settings),
//                 );
//         }
//     }

//     fn setup_system(mut commands: Commands, assets: Res<AssetServer>) {
//         // Create a default stylesheet. You can customize these as you wish
//         let sheet = Stylesheet::default().with_background(BackgroundColor(Color::BLACK));

//         commands.insert_resource(MenuState::new(
//             CustomState {
//                 sound_on: true,
//                 gamepads: Vec::new(),
//                 controls: [
//                     (0, ControlDevice::keyboard1()),
//                     (1, ControlDevice::keyboard2()),
//                     (2, ControlDevice::keyboard3()),
//                     (3, ControlDevice::keyboard4()),
//                 ]
//                 .into(),
//                 logo: assets.load("logo.png"),
//             },
//             Screens::Root,
//             Some(sheet),
//         ))
//     }

//     /// Whenever a new gamepad connects, get the known gamepads and their names
//     /// into our state
//     fn update_gamepads_system(
//         gamepads: Query<(Entity, &Name), With<Gamepad>>,
//         event: EventReader<GamepadAxisChangedEvent>,
//         menu_state: Option<ResMut<MenuState<Screens>>>,
//     ) {
//         let Some(mut menu_state) = menu_state else {
//             return;
//         };
//         if !event.is_empty() {
//             let gamepads: Vec<(Entity, String)> = gamepads
//                 .iter()
//                 .map(|(e,n)| {
//                     (
//                         e,
//                         String::from(n.as_str().to_owned())
//                     )
//                 })
//                 .collect();
            
//             // if menu_state.state().gamepads != gamepads {
//             //     menu_state.state_mut().gamepads = gamepads;
//             // }

//         }
//     }

//     /// The possible actions in our settings
//     #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//     enum Actions {
//         Close,
//         SoundOn,
//         SoundOff,
//         Control(usize, ControlDevice),
//     }

//     /// Handle the possible actions
//     impl ActionTrait for Actions {
//         type State = CustomState;
//         type Event = MyEvent;
//         fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
//             match self {
//                 Actions::Close => {
//                     event_writer.send(MyEvent::CloseSettings);
//                 }
//                 Actions::SoundOn => {
//                     state.sound_on = true;
//                 }
//                 Actions::SoundOff => {
//                     state.sound_on = false;
//                 }
//                 Actions::Control(p, d) => {
//                     state.controls.insert(*p, *d);
//                 }
//             }
//         }
//     }

//     /// All possible screens in our settings
//     #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//     enum Screens {
//         Root,
//         Controls,
//         Sound,
//         Player(usize),
//     }

//     impl ScreenTrait for Screens {
//         type Action = Actions;
//         type State = CustomState;
//         fn resolve(&self, state: &CustomState) -> Menu<Screens> {
//             match self {
//                 Screens::Root => root_menu(state),
//                 Screens::Controls => controls_menu(state),
//                 Screens::Sound => sound_menu(state),
//                 Screens::Player(p) => player_controls_menu(state, *p),
//             }
//         }
//     }

//     /// The `root` menu that is displayed first
//     fn root_menu(state: &CustomState) -> Menu<Screens> {
//         Menu::new(
//             "root",
//             vec![
//                 MenuItem::image(state.logo.clone()),
//                 MenuItem::headline("Menu"),
//                 MenuItem::action("Start", Actions::Close),
//                 MenuItem::screen("Sound", Screens::Sound).with_icon(MenuIcon::Sound),
//                 MenuItem::screen("Controls", Screens::Controls).with_icon(MenuIcon::Controls),
//             ],
//         )
//     }

//     /// This is displayed if the user selects `Sound` in the `root_menu`
//     fn sound_menu(state: &CustomState) -> Menu<Screens> {
//         Menu::new(
//             "sound",
//             vec![
//                 MenuItem::label("Toggles sound and music"),
//                 MenuItem::action("On", Actions::SoundOn).checked(state.sound_on),
//                 MenuItem::action("Off", Actions::SoundOff).checked(!state.sound_on),
//             ],
//         )
//     }

//     /// This is displayed if the user selects `Controls` in the `root_menu`
//     fn controls_menu(state: &CustomState) -> Menu<Screens> {
//         let mut players: Vec<usize> = state.controls.keys().copied().collect();
//         players.sort();
//         Menu::new(
//             "controls",
//             players
//                 .into_iter()
//                 .map(|player| MenuItem::screen(format!("Player {player}"), Screens::Player(player)))
//                 .collect(),
//         )
//     }

//     /// This is displayed if the user selects a player in the `controls_menu`
//     fn player_controls_menu(state: &CustomState, player: usize) -> Menu<Screens> {
//         let selected_control = state.controls[&player];
//         // Get the Keyboards
//         let mut entries: Vec<_> = vec![
//             ControlDevice::keyboard1(),
//             ControlDevice::keyboard2(),
//             ControlDevice::keyboard3(),
//             ControlDevice::keyboard4(),
//         ]
//         .iter()
//         .map(|kb| {
//             MenuItem::action(kb.to_string(), Actions::Control(player, *kb))
//                 .checked(kb.id() == selected_control.id())
//         })
//         .collect();

//         // Get the GamePads
//         for (pad, title) in &state.gamepads {
//             todo!("Fix");
//             let device = ControlDevice::Gamepad { gamepad_id: pad.id };
//             entries.push(
//                 MenuItem::action(title, Actions::Control(player, device))
//                     .checked(device.id() == selected_control.id()),
//             )
//         }

//         Menu::new("players", entries)
//     }

//     /// This allows to react to actions with custom bevy resources or eventwriters or queries.
//     /// In this example we use it to close the menu
//     fn event_reader(
//         mut commands: Commands,
//         mut event_reader: EventReader<MyEvent>,
//         mut next_state: ResMut<NextState<GameState>>,
//     ) {
//         for event in event_reader.read() {
//             match event {
//                 MyEvent::CloseSettings => {
//                     bevy_quickmenu::cleanup(&mut commands);
//                     next_state.set(GameState::Game);
//                 }
//             }
//         }
//     }

//     // Abstractions over control devices

//     #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//     pub enum ControlDevice {
//         Gamepad {
//             gamepad_id: usize,
//         },
//         Keyboard {
//             title: &'static str,
//             description: &'static str,
//             keyboard_id: usize,
//             left: KeyCode,
//             right: KeyCode,
//             action: KeyCode,
//         },
//     }

//     impl ControlDevice {
//         pub fn id(&self) -> usize {
//             match self {
//                 ControlDevice::Gamepad { gamepad_id, .. } => *gamepad_id,
//                 ControlDevice::Keyboard { keyboard_id, .. } => *keyboard_id,
//             }
//         }
//     }

//     impl std::fmt::Display for ControlDevice {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 ControlDevice::Gamepad { gamepad_id } => {
//                     f.write_fmt(format_args!("Gamepad {gamepad_id}",))
//                 }
//                 ControlDevice::Keyboard { title, .. } => f.write_fmt(format_args!("{title}")),
//             }
//         }
//     }

//     impl ControlDevice {
//         pub fn keyboard1() -> ControlDevice {
//             ControlDevice::Keyboard {
//                 title: "Keyboard 1",
//                 description: "Left / Right + M",
//                 keyboard_id: 42001,
//                 left: KeyCode::ArrowLeft,
//                 right: KeyCode::ArrowRight,
//                 action: KeyCode::KeyM,
//             }
//         }

//         pub fn keyboard2() -> ControlDevice {
//             ControlDevice::Keyboard {
//                 title: "Keyboard 2",
//                 description: "A / D + B",
//                 keyboard_id: 42002,
//                 left: KeyCode::KeyA,
//                 right: KeyCode::KeyD,
//                 action: KeyCode::KeyB,
//             }
//         }
//         pub fn keyboard3() -> ControlDevice {
//             ControlDevice::Keyboard {
//                 title: "Keyboard 3",
//                 description: "I / O + K",
//                 keyboard_id: 42003,
//                 left: KeyCode::KeyI,
//                 right: KeyCode::KeyO,
//                 action: KeyCode::KeyK,
//             }
//         }
//         pub fn keyboard4() -> ControlDevice {
//             ControlDevice::Keyboard {
//                 title: "Keyboard 4",
//                 description: "T / Y + H",
//                 keyboard_id: 42004,
//                 left: KeyCode::KeyT,
//                 right: KeyCode::KeyY,
//                 action: KeyCode::KeyH,
//             }
//         }
//     }
// }

// // Replicate a simple game to show how to go back to the
// // menu screen and show it again
// mod game {
//     use super::*;
//     pub struct Game;
//     impl Plugin for Game {
//         fn build(&self, app: &mut App) {
//             app.add_systems(OnEnter(GameState::Game), setup_system)
//                 .add_systems(Update, detect_close_system.run_if(GameState::is_game));
//             // .run();
//         }
//     }

//     #[derive(Component)]
//     struct GameComponent;

//     fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
//         commands.spawn(
//             Node {
//                 position_type: PositionType::Absolute,
//                 top: Val::Px(60.0),
//                 left: Val::Px(50.0),
//                 ..default()
//             }
//         ).with_child((
//             Text("Return Key to go back to menu".into()),
//             TextFont {
//                 font: asset_server.load("font.otf"),
//                 font_size: 30.0,
//                 ..Default::default()
//             },
//             TextColor::from(Color::WHITE),
            
//         ))
//         .insert(GameComponent);
//     }

//     fn detect_close_system(
//         mut commands: Commands,
//         keyboard_input: Res<ButtonInput<KeyCode>>,
//         mut next_state: ResMut<NextState<GameState>>,
//         game_items: Query<Entity, With<GameComponent>>,
//     ) {
//         if keyboard_input.just_pressed(KeyCode::Enter) {
//             for entity in game_items.iter() {
//                 commands.entity(entity).despawn_recursive();
//             }
//             next_state.set(GameState::Settings);
//         }
//     }
// }
