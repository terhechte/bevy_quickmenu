#![doc = include_str!("../Readme.md")]

// pub mod helpers;
mod navigation_menu;
pub mod style;
mod systems;
mod types;
mod widgets;

use bevy::prelude::*;
use style::Stylesheet;
use types::{CleanUpUI, MenuAssets};

use std::fmt::Debug;
use std::hash::Hash;

pub use navigation_menu::NavigationMenu;
pub use types::{
    ButtonComponent, Menu, MenuIcon, MenuItem, MenuOptions, MenuSelection, NavigationEvent,
    PrimaryMenu, RedrawEvent, RichTextEntry, Selections, VerticalMenuComponent,
};

/// The quickmenu plugin.
/// It requires multiple generic parameters in order to setup. A minimal example.
/// For a full explanation refer to the examples or the README.
/// ```
/// use bevy::prelude::*;
///
/// use bevy_quickmenu::{
///     style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuState, QuickMenuPlugin,
///     ScreenTrait,
/// };
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(BasicPlugin);
///         //.run();
/// }
///
/// /// This custom event can be emitted by the action handler (below) in order to
/// /// process actions with access to the bevy ECS
/// #[derive(Debug, Event)]
/// enum BasicEvent {
///     Close,
/// }
///
/// /// This state represents the UI. Mutations to this state (via `MenuState::state_mut`)
/// /// cause a re-render of the menu UI
/// #[derive(Debug, Clone, Default)]
/// struct BasicState {
///     boolean1: bool,
///     boolean2: bool,
/// }
///
/// pub struct BasicPlugin;
///
/// impl Plugin for BasicPlugin {
///     fn build(&self, app: &mut App) {
///         app
///             // Register a event that can be called from your action handler
///             .add_event::<BasicEvent>()
///             // The plugin
///             .add_plugins(QuickMenuPlugin::<Screens>::new())
///             // Some systems
///             .add_systems(Startup, setup)
///             .add_systems(Update, event_reader);
///     }
/// }
///
/// fn setup(mut commands: Commands) {
///     commands.spawn(Camera3dBundle::default());
///     // Create a default stylesheet. You can customize these as you wish
///     let sheet = Stylesheet::default();
///
///     commands.insert_resource(MenuState::new(
///         BasicState::default(),
///         Screens::Root,
///         Some(sheet),
///     ))
/// }
///
/// /// The possible actions in our settings
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum Actions {
///     Close,
///     Toggle1,
///     Toggle2,
/// }
///
/// /// Handle the possible actions
/// impl ActionTrait for Actions {
///     type State = BasicState;
///     type Event = BasicEvent;
///     fn handle(&self, state: &mut BasicState, event_writer: &mut EventWriter<BasicEvent>) {
///         match self {
///             Actions::Close => event_writer.send(BasicEvent::Close),
///             Actions::Toggle1 => state.boolean1 = !state.boolean1,
///             Actions::Toggle2 => state.boolean2 = !state.boolean2,
///         }
///     }
/// }
///
/// /// All possible screens in our example
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum Screens {
///     Root,
///     Booleans,
/// }
///
/// /// Map from from `Screens` to the actual menu
/// impl ScreenTrait for Screens {
///     type Action = Actions;
///     type State = BasicState;
///     fn resolve(&self, state: &BasicState) -> Menu<Screens> {
///         match self {
///             Screens::Root => root_menu(state),
///             Screens::Booleans => boolean_menu(state),
///         }
///     }
/// }
///
/// /// The `root` menu that is displayed first
/// fn root_menu(_state: &BasicState) -> Menu<Screens> {
///     Menu::new(
///         "root",
///         vec![
///             MenuItem::headline("Basic Example"),
///             MenuItem::action("Close", Actions::Close).with_icon(MenuIcon::Back),
///             MenuItem::label("A submenu"),
///             MenuItem::screen("Boolean", Screens::Booleans),
///         ],
///     )
/// }
///
/// /// The boolean menu which is accessed from the `Screens::Boolean` entry in the root_menu
/// fn boolean_menu(state: &BasicState) -> Menu<Screens> {
///     Menu::new(
///         "boolean",
///         vec![
///             MenuItem::label("Toggles some booleans"),
///             MenuItem::action("Toggle Boolean 1", Actions::Toggle1).checked(state.boolean1),
///             MenuItem::action("Toggle Boolean 2", Actions::Toggle2).checked(state.boolean2),
///         ],
///     )
/// }
///
/// /// This allows to react to actions with custom bevy resources or eventwriters or queries.
/// /// In this example we use it to close the menu
/// fn event_reader(mut commands: Commands, mut event_reader: EventReader<BasicEvent>) {
///     for event in event_reader.read() {
///         match event {
///             BasicEvent::Close => bevy_quickmenu::cleanup(&mut commands),
///         }
///     }
/// }
/// ```
pub struct QuickMenuPlugin<S>
where
    S: ScreenTrait + 'static,
{
    s: std::marker::PhantomData<S>,
    options: Option<MenuOptions>,
}

impl<S> QuickMenuPlugin<S>
where
    S: ScreenTrait + 'static,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            s: Default::default(),
            options: None,
        }
    }

    pub fn with_options(options: MenuOptions) -> Self {
        Self {
            s: Default::default(),
            options: Some(options),
        }
    }
}

impl<State, A, S> Plugin for QuickMenuPlugin<S>
where
    State: 'static + Send + Sync,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A, State = State> + 'static,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.options.unwrap_or_default())
            .init_resource::<MenuAssets>()
            .insert_resource(Selections::default())
            .add_event::<NavigationEvent>()
            .add_event::<RedrawEvent>()
            .add_systems(
                Update,
                systems::cleanup_system::<S>.run_if(resource_exists::<CleanUpUI>()),
            )
            .add_systems(
                Update,
                (
                    systems::mouse_system::<S>.run_if(resource_exists::<MenuState<S>>()),
                    systems::input_system::<S>.run_if(resource_exists::<MenuState<S>>()),
                    systems::redraw_system::<S>.run_if(resource_exists::<MenuState<S>>()),
                    systems::keyboard_input_system.run_if(resource_exists::<MenuState<S>>()),
                ),
            );
    }
}

/// Remove the menu
pub fn cleanup(commands: &mut Commands) {
    commands.init_resource::<CleanUpUI>();
}

/// A type conforming to this trait is used to handle the events that
/// are generated as the user interacts with the menu
pub trait ActionTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type State;
    type Event: Event + Send + Sync + 'static;
    fn handle(&self, state: &mut Self::State, event_writer: &mut EventWriter<Self::Event>);
}

/// Each Menu / Screen uses this trait to define which menu items lead
/// to which other screens
pub trait ScreenTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type Action: ActionTrait<State = Self::State>;
    type State: Send + Sync + 'static;
    fn resolve(&self, state: &<<Self as ScreenTrait>::Action as ActionTrait>::State) -> Menu<Self>;
}

/// The primary state resource of the menu
#[derive(Resource)]
pub struct MenuState<S>
where
    S: ScreenTrait + 'static,
{
    menu: NavigationMenu<S>,
    pub initial_render_done: bool,
}

impl<S> MenuState<S>
where
    S: ScreenTrait + 'static,
{
    pub fn new(state: S::State, screen: S, sheet: Option<Stylesheet>) -> Self {
        Self {
            menu: NavigationMenu::new(state, screen, sheet),
            initial_render_done: false,
        }
    }

    /// Get a mutable reference to the state in order to change it.
    /// Changing something here will cause a re-render in the next frame.
    /// Due to the way bevy works, just getting this reference, without actually performing
    /// a change is enough to cause a re-render.
    pub fn state_mut(&mut self) -> &mut S::State {
        &mut self.menu.state
    }

    /// Can a immutable reference to the state.
    pub fn state(&self) -> &S::State {
        &self.menu.state
    }
}
