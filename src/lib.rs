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
use bevy_egui::{
    egui::{Id, Ui},
    EguiPlugin,
};
use style::Stylesheet;

use std::fmt::Debug;
use std::hash::Hash;

pub use bevy_egui::egui;
pub use navigation_menu::NavigationMenu;
pub use types::{CursorDirection, CustomFontData, MenuIcon, MenuItem, MenuSelection};

pub struct Menu<A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub id: Id,
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
        state: &mut <<Self as ScreenTrait>::Action as ActionTrait>::State,
    ) -> Menu<Self::Action, Self, <<Self as ScreenTrait>::Action as ActionTrait>::State>;
}

pub fn make_menu<State, A, S>(
    ui: &mut Ui,
    id: Id,
    cursor_direction: Option<CursorDirection>,
    items: &[MenuItem<State, A, S>],
    stylesheet: &Stylesheet,
) -> Option<MenuSelection<A, S, State>>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    let mut selection: Option<MenuSelection<A, S, State>> = None;
    ui.add(widgets::VerticalMenu {
        id,
        cursor_direction,
        items,
        selection: &mut selection,
        stylesheet,
    });
    selection
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

    pub fn state(&mut self) -> &mut State {
        &mut self.menu.state
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
        app.add_plugin(EguiPlugin)
            .add_event::<CursorDirection>()
            .add_startup_system(crate::systems::setup_menu_system::<State, A, S>)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(resource_exists::<State, A, S>)
                    .with_system(crate::systems::keyboard_input_system)
                    .with_system(crate::systems::input_system::<State, A, S>)
                    .with_system(crate::systems::ui_settings_system::<State, A, S>),
            );
    }
}

fn resource_exists<State, A, S>(resource: Option<Res<SettingsState<State, A, S>>>) -> ShouldRun
where
    State: Send + Sync + 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    resource.is_some().into()
}
