use crate::{ActionTrait, ScreenTrait};
use bevy::prelude::Resource;
use bevy_egui::egui::WidgetText;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorDirection {
    Up,
    Down,
    Select,
    Back,
}

pub enum MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Screen(WidgetText, S),
    Action(WidgetText, A),
}

impl<State, A, S> MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn screen(s: impl Into<WidgetText>, screen: S) -> Self {
        MenuItem::Screen(s.into(), screen)
    }

    pub fn action(s: impl Into<WidgetText>, action: A) -> Self {
        MenuItem::Action(s.into(), action)
    }

    pub(crate) fn as_selection(&self) -> MenuSelection<A, S, State> {
        match self {
            MenuItem::Screen(_, a) => MenuSelection::Screen(*a),
            MenuItem::Action(_, a) => MenuSelection::Action(*a),
        }
    }

    pub(crate) fn text(&self) -> &WidgetText {
        match self {
            MenuItem::Screen(t, _) => t,
            MenuItem::Action(t, _) => t,
        }
    }
}

#[derive(Debug)]
pub enum MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Action(A),
    Screen(S),
}

impl<A, S, State> Clone for MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Action(arg0) => Self::Action(*arg0),
            Self::Screen(arg0) => Self::Screen(*arg0),
        }
    }
}

impl<A, S, State> PartialEq for MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MenuSelection::Action(a1), MenuSelection::Action(a2)) => a1 == a2,
            (MenuSelection::Screen(s1), MenuSelection::Screen(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Resource)]
pub struct CustomFontData(pub Option<&'static [u8]>);
