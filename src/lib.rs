pub mod helpers;
mod navigation_menu;
pub mod style;
mod systems;
mod types;
mod widgets;

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{EventWriter, Plugin, Res, Resource, SystemSet},
};
use style::Stylesheet;

use std::fmt::Debug;
use std::hash::Hash;

pub use navigation_menu::NavigationMenu;
pub use types::{
    CustomFontData, MenuIcon, MenuItem, MenuSelection, NavigationEvent, RedrawEvent, Selections,
};

pub struct Menu<A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub id: &'static str,
    pub entries: Vec<MenuItem<State, A, S>>,
}

pub trait ActionTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type State;
    type Event: Send + Sync + 'static;
    fn handle(&self, state: &mut Self::State, event_writer: &mut EventWriter<Self::Event>);
}

pub trait ScreenTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type Action: ActionTrait;
    fn resolve(
        &self,
        state: &<<Self as ScreenTrait>::Action as ActionTrait>::State,
    ) -> Menu<Self::Action, Self, <<Self as ScreenTrait>::Action as ActionTrait>::State>;
}

#[derive(Resource)]
pub struct SettingsState<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    menu: NavigationMenu<State, A, S>,
}

impl<State, A, S> SettingsState<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn new(state: State, screen: S, sheet: Option<Stylesheet>) -> Self {
        Self {
            menu: NavigationMenu::new(state, screen, sheet),
        }
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.menu.state
    }

    pub fn state(&self) -> &State {
        &self.menu.state
    }
}

pub struct QuickMenuPlugin<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    state: std::marker::PhantomData<State>,
    a: std::marker::PhantomData<A>,
    s: std::marker::PhantomData<S>,
}

impl<State, A, S> Default for QuickMenuPlugin<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    fn default() -> Self {
        Self {
            state: Default::default(),
            a: Default::default(),
            s: Default::default(),
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
        app.add_event::<NavigationEvent>()
            .add_event::<RedrawEvent>()
            .add_startup_system(crate::systems::setup_menu_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(resource_exists::<State, A, S>)
                    .with_system(crate::systems::keyboard_input_system)
                    .with_system(crate::systems::input_system::<State, A, S>)
                    .with_system(crate::systems::mouse_system::<State, A, S>)
                    .with_system(crate::systems::redraw_system::<State, A, S>),
            );
    }
}

pub fn resource_exists<State, A, S>(resource: Option<Res<SettingsState<State, A, S>>>) -> ShouldRun
where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    resource.is_some().into()
}
