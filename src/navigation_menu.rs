use bevy::prelude::EventWriter;
use bevy_egui::egui::Ui;
use std::fmt::Debug;

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
    state: State,
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn new(state: State, root: S) -> Self {
        Self {
            stack: vec![root],
            next_direction: None,
            state,
        }
    }

    pub fn next(&mut self, direction: CursorDirection) {
        self.next_direction = Some(direction);
    }
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
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
                if let Some(next) = entry.resolve(ui, &mut self.state, cursor_direction) {
                    if is_last {
                        next_menu = Some(next);
                    }
                }
            }
        });

        if let Some(n) = next_menu {
            match n {
                MenuSelection::Action(a) => a.handle(&mut self.state, event_writer),
                MenuSelection::Screen(s) => self.stack.push(s),
            }
        }
    }
}
