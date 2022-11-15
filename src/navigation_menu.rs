use bevy::prelude::EventWriter;
// use bevy_egui::egui::Ui;
use bevy::prelude::*;
use std::fmt::Debug;

use crate::{style::Stylesheet, types::QuickMenuComponent, Selections};

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
    /// Any user input (cursor keys, gamepad) is set here
    next_direction: Option<NavigationEvent>,
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

    pub fn next(&mut self, direction: NavigationEvent) {
        self.next_direction = Some(direction);
    }
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn show(
        &self,
        selections: &Selections,
        commands: &mut Commands,
        // event_writer: &mut EventWriter<A::Event>,
    ) {
        // let next_direction = self.next_direction.take();
        // let next_direction = None;

        // Can't pop the root
        // if self.stack.len() > 1 {
        //     if let Some(NavigationEvent::Back) = next_direction {
        //         self.stack.pop();
        //     }
        // }

        let mut next_menu: Option<MenuSelection<A, S, State>> = None;

        commands
            .spawn(NodeBundle {
                style: Style {
                    // size: Size::new(Val::Percent(100.0), Val::Px(30.)),
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(self.stylesheet.vertical_spacing)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for (index, entry) in self.stack.iter().enumerate() {
                    let is_last = (index + 1) == self.stack.len();
                    // let cursor_direction = if is_last { next_direction } else { None };
                    let menu_desc = entry.resolve(&self.state);
                    // if let Some(next) = {
                    // let mut selection: Option<MenuSelection<A, S, State>> = None;
                    super::widgets::VerticalMenu {
                        id: menu_desc.id,
                        // cursor_direction,
                        items: &menu_desc.entries,
                        // selection: &mut selection,
                        stylesheet: &self.stylesheet,
                    }
                    .build(selections, parent, is_last);
                    //     selection
                    // } {
                    //     if is_last {
                    //         next_menu = Some(next);
                    //     }
                    // }

                    // if let Some(next) = make_menu(
                    //     parent,
                    //     menu_desc.id,
                    //     cursor_direction,
                    //     &menu_desc.entries,
                    //     &self.stylesheet,
                    //     selections,
                    // ) {
                    //     if is_last {
                    //         next_menu = Some(next);
                    //     }
                    // }
                    // if !is_last {
                    //     // FIXME: SPace
                    //     //ui.add_space(self.stylesheet.horizontal_spacing);
                    // }
                }
            })
            .insert(QuickMenuComponent);

        // ui.horizontal(|ui| {

        // });
        // if let Some(n) = next_menu {
        //     println!("next action {n:?}");
        //     self.handle_selection(&n, event_writer);
        //     true
        // } else {
        //     false
        // }
    }

    pub fn apply_event(
        &mut self,
        event: &NavigationEvent,
        selections: &mut Selections,
    ) -> Option<MenuSelection<A, S, State>> {
        if self.stack.len() > 1 {
            if &NavigationEvent::Back == event {
                self.stack.pop();
            }
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
            MenuSelection::Screen(s) => self.stack.push(s.clone()),
            MenuSelection::None => (),
        }
    }
}
