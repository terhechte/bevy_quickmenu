//! Basic Example
//! A basic example that shows how to open a menu with boolen values
//! that can be toggled
use bevy::prelude::*;

use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuState, QuickMenuPlugin,
    ScreenTrait,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BasicPlugin)
        .run();
}

/// This custom event can be emitted by the action handler (below) in order to
/// process actions with access to the bevy ECS
#[derive(Debug)]
enum BasicEvent {
    Close,
}

/// This state represents the UI. Mutations to this state (via `MenuState::state_mut`)
/// cause a re-render of the menu UI
#[derive(Debug, Clone, Default)]
struct BasicState {
    boolean1: bool,
    boolean2: bool,
}

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register a event that can be called from your action handler
            .add_event::<BasicEvent>()
            // The plugin
            .add_plugin(QuickMenuPlugin::<BasicState, Actions, Screens>::new())
            // Some systems
            .add_startup_system(setup)
            .add_system(event_reader);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    // Create a default stylesheet. You can customize these as you wish
    let sheet = Stylesheet::default();

    commands.insert_resource(MenuState::new(
        BasicState::default(),
        Screens::Root,
        Some(sheet),
    ))
}

/// The possible actions in our settings
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Close,
    Toggle1,
    Toggle2,
}

/// Handle the possible actions
impl ActionTrait for Actions {
    type State = BasicState;
    type Event = BasicEvent;
    fn handle(&self, state: &mut BasicState, event_writer: &mut EventWriter<BasicEvent>) {
        match self {
            Actions::Close => event_writer.send(BasicEvent::Close),
            Actions::Toggle1 => state.boolean1 = !state.boolean1,
            Actions::Toggle2 => state.boolean2 = !state.boolean2,
        }
    }
}

/// All possible screens in our example
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Booleans,
}

/// Map from from `Screens` to the actual menu
impl ScreenTrait for Screens {
    type Action = Actions;
    fn resolve(&self, state: &BasicState) -> Menu<Actions, Screens, BasicState> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Booleans => boolean_menu(state),
        }
    }
}

/// The `root` menu that is displayed first
fn root_menu(_state: &BasicState) -> Menu<Actions, Screens, BasicState> {
    Menu::new(
        "root",
        vec![
            MenuItem::headline("Basic Example"),
            MenuItem::action("Close", Actions::Close).with_icon(MenuIcon::Back),
            MenuItem::label("A submenu"),
            MenuItem::screen("Boolean", Screens::Booleans),
        ],
    )
}

/// The boolean menu which is accessed from the `Screens::Boolean` entry in the root_menu
fn boolean_menu(state: &BasicState) -> Menu<Actions, Screens, BasicState> {
    Menu::new(
        "boolean",
        vec![
            MenuItem::label("Toggles some booleans"),
            MenuItem::action("Toggle Boolean 1", Actions::Toggle1).checked(state.boolean1),
            MenuItem::action("Toggle Boolean 2", Actions::Toggle2).checked(state.boolean2),
        ],
    )
}

/// This allows to react to actions with custom bevy resources or eventwriters or queries.
/// In this example we use it to close the menu
fn event_reader(mut commands: Commands, mut event_reader: EventReader<BasicEvent>) {
    for event in event_reader.iter() {
        match event {
            BasicEvent::Close => bevy_quickmenu::cleanup(&mut commands),
        }
    }
}
