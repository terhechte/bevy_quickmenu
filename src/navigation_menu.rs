use bevy::prelude::EventWriter;
use bevy::prelude::*;
use std::fmt::Debug;

use crate::{
    style::Stylesheet,
    types::{MenuAssets, QuickMenuComponent},
    Selections,
};

use super::{
    types::{MenuSelection, NavigationEvent},
    ActionTrait, ScreenTrait,
};

#[derive(Debug)]
pub struct NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    /// The internal stack of menu screens
    stack: Vec<S>,
    /// The custom state
    pub(crate) state: State,
    /// The style to use
    pub(crate) stylesheet: Stylesheet,
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn new(state: State, root: S, sheet: Option<Stylesheet>) -> Self {
        Self {
            stack: vec![root],
            state,
            stylesheet: sheet.unwrap_or_default(),
        }
    }
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn show(&self, assets: &MenuAssets, selections: &Selections, commands: &mut Commands) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(self.stylesheet.vertical_spacing)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for entry in self.stack.iter() {
                    let menu_desc = entry.resolve(&self.state);
                    super::widgets::VerticalMenu {
                        id: menu_desc.id,
                        items: &menu_desc.entries,
                        stylesheet: &self.stylesheet,
                        assets,
                    }
                    .build(selections, parent);
                }
            })
            .insert(QuickMenuComponent);
    }

    pub fn apply_event(
        &mut self,
        event: &NavigationEvent,
        selections: &mut Selections,
    ) -> Option<MenuSelection<A, S, State>> {
        if self.stack.len() > 1 && &NavigationEvent::Back == event {
            self.stack.pop();
        }
        for (index, entry) in self.stack.iter().enumerate() {
            let is_last = (index + 1) == self.stack.len();
            let menu_desc = entry.resolve(&self.state);
            if is_last {
                return super::widgets::VerticalMenu::apply_event(
                    event,
                    menu_desc.id,
                    &menu_desc.entries,
                    selections,
                );
            }
        }
        None
    }

    pub fn handle_selection(
        &mut self,
        selection: &MenuSelection<A, S, State>,

        event_writer: &mut EventWriter<A::Event>,
    ) {
        match selection {
            MenuSelection::Action(a) => a.handle(&mut self.state, event_writer),
            MenuSelection::Screen(s) => self.stack.push(*s),
            MenuSelection::None => (),
        }
    }

    pub fn pop_to_selection(&mut self, selection: &MenuSelection<A, S, State>) {
        let mut found = false;
        let mut items = 0;
        for entry in self.stack.iter() {
            let menu_desc = entry.resolve(&self.state);

            if found {
                items += 1;
            }

            for entry in menu_desc.entries {
                if &entry.as_selection() == selection {
                    found = true;
                }
            }
        }
        if self.stack.len() > 1 {
            for _ in 0..items {
                self.stack.pop();
            }
        }
    }
}
