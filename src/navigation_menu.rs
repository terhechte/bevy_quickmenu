//! Navigation Menu
//! This is the primary horizontal menu which is used to host the various
//! screens / vertical menus.
use bevy::prelude::EventWriter;
use bevy::prelude::*;
use std::fmt::Debug;

use crate::{
    style::Stylesheet,
    types::{MenuAssets, PrimaryMenu, QuickMenuComponent},
    Selections,
};

use super::{
    types::{MenuSelection, NavigationEvent},
    ActionTrait, ScreenTrait,
};

#[derive(Debug)]
pub struct NavigationMenu<S>
where
    S: ScreenTrait,
{
    /// The internal stack of menu screens
    stack: Vec<S>,
    /// The custom state
    pub(crate) state: S::State,
    /// The style to use
    pub(crate) stylesheet: Stylesheet,
}

impl<S> NavigationMenu<S>
where
    S: ScreenTrait,
{
    pub fn new(state: S::State, root: S, sheet: Option<Stylesheet>) -> Self {
        Self {
            stack: vec![root],
            state,
            stylesheet: sheet.unwrap_or_default(),
        }
    }
}

impl<S> NavigationMenu<S>
where
    S: ScreenTrait + 'static,
{
    pub fn show(&self, assets: &MenuAssets, selections: &Selections, commands: &mut Commands) {
        let style = self
            .stylesheet
            .style
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Style {
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(self.stylesheet.vertical_spacing)),
                ..default()
            });

        let background_color = self
            .stylesheet
            .background
            .unwrap_or_else(|| Color::NONE.into());

        commands
            .spawn(NodeBundle {
                style,
                background_color,
                ..default()
            })
            .insert(PrimaryMenu)
            .with_children(|parent| {
                for entry in self.stack.iter() {
                    let menu_desc = entry.resolve(&self.state);
                    super::widgets::VerticalMenu {
                        id: menu_desc.id,
                        items: &menu_desc.entries,
                        stylesheet: &self.stylesheet,
                        assets,
                        style: menu_desc.style.as_ref(),
                        background: menu_desc.background.as_ref(),
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
    ) -> Option<MenuSelection<S>> {
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
        selection: &MenuSelection<S>,
        event_writer: &mut EventWriter<<<S as ScreenTrait>::Action as ActionTrait>::Event>,
    ) {
        match selection {
            MenuSelection::Action(a) => a.handle(&mut self.state, event_writer),
            MenuSelection::Screen(s) => self.stack.push(*s),
            MenuSelection::None => (),
        }
    }

    pub fn pop_to_selection(&mut self, selection: &MenuSelection<S>) {
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
