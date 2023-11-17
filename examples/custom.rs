//! Customize the Quickmenu
//!
//! This example shows how to customize the quickmenu via:
//! - Custom `Style` entries
//! - Customzing the button ControlStates in the Stylesheet
//! - Loading a custom font
//! - Using a custom icon
//! - Using Rich Text
//! - Using background colors for the menus

use bevy::prelude::*;

use bevy_quickmenu::{
    style::{ControlState, StyleEntry, Stylesheet},
    ActionTrait, Menu, MenuIcon, MenuItem, MenuOptions, MenuState, QuickMenuPlugin, RichTextEntry,
    ScreenTrait,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasicPlugin)
        .run();
}

/// This custom event can be emitted by the action handler (below) in order to
/// process actions with access to the bevy ECS
#[derive(Debug, Event)]
enum BasicEvent {
    Close,
}

/// This state represents the UI. Mutations to this state (via `SettingsState::state_mut`)
/// cause a re-render of the menu UI
#[derive(Debug, Clone, Default)]
struct BasicState {
    boolean1: bool,
    boolean2: bool,
    custom_icon: Handle<Image>,
}

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn build(&self, app: &mut App) {
        // Load a custom font
        let options = MenuOptions {
            font: Some("font.otf"),
            ..Default::default()
        };

        app
            // Register a event that can be called from your action handler
            .add_event::<BasicEvent>()
            // The plugin
            .add_plugins(QuickMenuPlugin::<Screens>::with_options(options))
            // Some systems
            .add_systems(Startup, setup)
            .add_systems(Update, event_reader);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle::default());
    // Create a customized stylesheet
    let mut button_style = StyleEntry::button();
    button_style.size = 25.0;
    button_style.selected = ControlState {
        fg: Color::YELLOW,
        bg: Color::RED,
    };

    let sheet = Stylesheet {
        button: button_style,
        ..Default::default()
    }
    .with_background(BackgroundColor(Color::BISQUE));

    // Load custom icons
    let state = BasicState {
        custom_icon: asset_server.load("Custom.png"),
        ..Default::default()
    };

    commands.insert_resource(MenuState::new(state, Screens::Root, Some(sheet)));
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
    type State = BasicState;
    fn resolve(&self, state: &BasicState) -> Menu<Screens> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Booleans => boolean_menu(state),
        }
    }
}

/// The `root` menu that is displayed first
fn root_menu(state: &BasicState) -> Menu<Screens> {
    Menu::new(
        "root",
        vec![
            MenuItem::headline([
                RichTextEntry::new("Rich "),
                RichTextEntry::new_color("Text ", Color::RED),
                RichTextEntry::new_color("!", Color::YELLOW),
            ]),
            MenuItem::action("Close", Actions::Close).with_icon(MenuIcon::Back),
            MenuItem::label("Use a custom Icon"),
            MenuItem::screen("Boolean", Screens::Booleans)
                .with_icon(MenuIcon::Other(state.custom_icon.clone())),
        ],
    )
    .with_background(BackgroundColor(Color::BLACK))
}

/// The boolean menu which is accessed from the `Screens::Boolean` entry in the root_menu
fn boolean_menu(state: &BasicState) -> Menu<Screens> {
    Menu::new(
        "boolean",
        vec![
            MenuItem::label("Right-Align the elements"),
            MenuItem::action("Toggle 1", Actions::Toggle1).checked(state.boolean1),
            MenuItem::action("Toggle Boolean 2", Actions::Toggle2).checked(state.boolean2),
        ],
    )
    .with_background(BackgroundColor(Color::NAVY))
    .with_style(Style {
        align_items: AlignItems::FlexEnd,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    })
}

/// This allows to react to actions with custom bevy resources or eventwriters or queries.
/// In this example we use it to close the menu
fn event_reader(mut commands: Commands, mut event_reader: EventReader<BasicEvent>) {
    for event in event_reader.read() {
        match event {
            BasicEvent::Close => bevy_quickmenu::cleanup(&mut commands),
        }
    }
}
