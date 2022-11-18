#![doc = include_str!("../README.md")]

// pub mod helpers;
mod navigation_menu;
pub mod style;
mod systems;
mod types;
mod widgets;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
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
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum Actions {
///     SoundOn,
///     SoundOff,
/// }
///
/// #[derive(Debug)]
/// enum MyEvent { SoundChanged }
///
/// impl ActionTrait for Actions {
///    type State = CustomState;
///    type Event = MyEvent;
///    fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
///         // handle action
///    }
/// }
///
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum Screens {
///     Root,
///     Sound,
/// }
///
/// impl ScreenTrait for Screens {
///     fn resolve(&self, state: &CustomState) -> Menu<Actions, Screens, CustomState> {
///         root_menu(state)
///     }
/// }
///
/// fn root_menu(_state: &CustomState) -> Menu<Actions, Screens, CustomState> {
///     Menu {
///         id: "root",
///         entries: vec![
///             MenuItem::headline("Sound Control"),
///             MenuItem::action("Sound On", Actions::SoundOn),
///             MenuItem::screen("Sound Off", Actions::SoundOff),
///         ]
///     }
/// }
///
/// #[derive(Debug, Clone)]
/// struct CustomState { sound_on: bool }
///
/// impl Plugin for MyApp {
///   fn build(&self, app: &mut App) {
///     app
///         .add_event::<MyEvent>()
///         .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::default())
///   }
/// }
/// ```
pub struct QuickMenuPlugin<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    state: std::marker::PhantomData<State>,
    a: std::marker::PhantomData<A>,
    s: std::marker::PhantomData<S>,
    options: Option<MenuOptions>,
}

impl<State, A, S> QuickMenuPlugin<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            state: Default::default(),
            a: Default::default(),
            s: Default::default(),
            options: None,
        }
    }

    pub fn with_options(options: MenuOptions) -> Self {
        Self {
            state: Default::default(),
            a: Default::default(),
            s: Default::default(),
            options: Some(options),
        }
    }
}

impl<State, A, S> Plugin for QuickMenuPlugin<State, A, S>
where
    State: 'static + Send + Sync,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.options.unwrap_or_default())
            .init_resource::<MenuAssets>()
            .insert_resource(Selections::default())
            .add_event::<NavigationEvent>()
            .add_event::<RedrawEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(resource_exists::<CleanUpUI>)
                    .with_system(systems::cleanup_system::<State, A, S>),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(resource_exists::<MenuState<State, A, S>>)
                    .with_system(crate::systems::keyboard_input_system)
                    .with_system(crate::systems::input_system::<State, A, S>)
                    .with_system(crate::systems::mouse_system::<State, A, S>)
                    .with_system(crate::systems::redraw_system::<State, A, S>),
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
    type Event: Send + Sync + 'static;
    fn handle(&self, state: &mut Self::State, event_writer: &mut EventWriter<Self::Event>);
}

/// Each Menu / Screen uses this trait to define which menu items lead
/// to which other screens
pub trait ScreenTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type Action: ActionTrait;
    fn resolve(
        &self,
        state: &<<Self as ScreenTrait>::Action as ActionTrait>::State,
    ) -> Menu<Self::Action, Self, <<Self as ScreenTrait>::Action as ActionTrait>::State>;
}

/// The primary state resource of the menu
#[derive(Resource)]
pub struct MenuState<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    menu: NavigationMenu<State, A, S>,
    pub initial_render_done: bool,
}

impl<State, A, S> MenuState<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn new(state: State, screen: S, sheet: Option<Stylesheet>) -> Self {
        Self {
            menu: NavigationMenu::new(state, screen, sheet),
            initial_render_done: false,
        }
    }

    /// Get a mutable reference to the state in order to change it.
    /// Changing something here will cause a re-render in the next frame.
    /// Due to the way bevy works, just getting this reference, without actually performing
    /// a change is enough to cause a re-render.
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.menu.state
    }

    /// Can a immutable reference to the state.
    pub fn state(&self) -> &State {
        &self.menu.state
    }
}

/// Helper to only run a system in specific circumstances
pub fn resource_exists<T: Resource>(resource: Option<Res<T>>) -> ShouldRun {
    resource.is_some().into()
}
