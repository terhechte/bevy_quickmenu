use bevy::prelude::EventWriter;
use bevy_egui::egui::Ui;
use std::fmt::Debug;

use crate::{make_menu, style::Stylesheet};

use super::{
    types::{CursorDirection, MenuSelection},
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
    /// Any user input (cursor keys, gamepad) is set here
    next_direction: Option<CursorDirection>,
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
            next_direction: None,
            state,
            stylesheet: sheet.unwrap_or_default(),
        }
    }

    pub fn next(&mut self, direction: CursorDirection) {
        self.next_direction = Some(direction);
    }
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn show(&mut self, ui: &mut Ui, event_writer: &mut EventWriter<A::Event>) {
        let next_direction = self.next_direction.take();

        if let Some(CursorDirection::Back) = next_direction {
            self.stack.pop();
        }

        let mut next_menu: Option<MenuSelection<A, S, State>> = None;

        ui.horizontal(|ui| {
            for (index, entry) in self.stack.iter().enumerate() {
                let is_last = (index + 1) == self.stack.len();
                let cursor_direction = if is_last { next_direction } else { None };
                let menu_desc = entry.resolve(&mut self.state);
                if let Some(next) = make_menu(
                    ui,
                    menu_desc.id,
                    cursor_direction,
                    &menu_desc.entries,
                    &self.stylesheet,
                ) {
                    if is_last {
                        next_menu = Some(next);
                    }
                }
                if !is_last {
                    ui.add_space(self.stylesheet.horizontal_spacing);
                }
            }
        });

        if let Some(n) = next_menu {
            match n {
                MenuSelection::Action(a) => a.handle(&mut self.state, event_writer),
                MenuSelection::Screen(s) => self.stack.push(s),
                MenuSelection::None => (),
            }
        }
    }
}
