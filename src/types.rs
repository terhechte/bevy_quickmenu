use crate::{ActionTrait, ScreenTrait};
use bevy::{
    prelude::{AssetServer, Component, Handle, Image, Resource, TextBundle},
    text::TextStyle,
    ui::UiImage,
    utils::HashMap,
};

#[derive(Component)]
pub struct QuickMenuComponent;

#[derive(Resource, Default)]
pub struct Selections(pub HashMap<&'static str, usize>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationEvent {
    Up,
    Down,
    Select,
    Back,
}

pub struct RedrawEvent;

pub enum MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Screen(WidgetText, MenuIcon, S),
    Action(WidgetText, MenuIcon, A),
    Label(WidgetText, MenuIcon),
    Headline(WidgetText, MenuIcon),
}

impl<State, A, S> MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn screen(s: impl Into<WidgetText>, screen: S) -> Self {
        MenuItem::Screen(s.into(), MenuIcon::None, screen)
    }

    pub fn action(s: impl Into<WidgetText>, action: A) -> Self {
        MenuItem::Action(s.into(), MenuIcon::None, action)
    }

    pub fn label(s: impl Into<WidgetText>) -> Self {
        MenuItem::Label(s.into(), MenuIcon::None)
    }

    pub fn headline(s: impl Into<WidgetText>) -> Self {
        MenuItem::Headline(s.into(), MenuIcon::None)
    }

    pub fn with_icon(self, icon: MenuIcon) -> Self {
        match self {
            MenuItem::Screen(a, _, b) => MenuItem::Screen(a, icon, b),
            MenuItem::Action(a, _, b) => MenuItem::Action(a, icon, b),
            MenuItem::Label(a, _) => MenuItem::Label(a, icon),
            MenuItem::Headline(a, _) => MenuItem::Headline(a, icon),
        }
    }

    pub fn checked(self, checked: bool) -> Self {
        if checked {
            self.with_icon(MenuIcon::Checked)
        } else {
            self.with_icon(MenuIcon::Unchecked)
        }
    }

    pub(crate) fn as_selection(&self) -> MenuSelection<A, S, State> {
        match self {
            MenuItem::Screen(_, _, a) => MenuSelection::Screen(*a),
            MenuItem::Action(_, _, a) => MenuSelection::Action(*a),
            MenuItem::Label(_, _) => MenuSelection::None,
            MenuItem::Headline(_, _) => MenuSelection::None,
        }
    }

    pub(crate) fn is_selectable(&self) -> bool {
        !matches!(self, MenuItem::Label(_, _) | MenuItem::Headline(_, _))
    }
}

impl<State, A, S> std::fmt::Debug for MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Screen(arg0, _, _) => f.debug_tuple("Screen").field(&arg0.text()).finish(),
            Self::Action(arg0, _, _) => f.debug_tuple("Action").field(&arg0.text()).finish(),
            Self::Label(arg0, _) => f.debug_tuple("Label").field(&arg0.text()).finish(),
            Self::Headline(arg0, _) => f.debug_tuple("Headline").field(&arg0.text()).finish(),
        }
    }
}

pub enum MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Action(A),
    Screen(S),
    None,
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
            Self::None => Self::None,
        }
    }
}

impl<A, S, State> std::fmt::Debug for MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(arg0) => f.debug_tuple("Action").field(&arg0).finish(),
            Self::Screen(arg0) => f.debug_tuple("Screen").field(&arg0).finish(),
            Self::None => f.debug_tuple("None").finish(),
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
            (MenuSelection::None, MenuSelection::None) => true,
            _ => false,
        }
    }
}

pub enum MenuIcon {
    None,
    Checked,
    Unchecked,
    Back,
    Controls,
    Sound,
    Players,
    Settings,
    Other(String, bool),
}

impl MenuIcon {
    pub(crate) fn resolve_icon(&self, asset_server: &AssetServer) -> Option<Handle<Image>> {
        match self {
            MenuIcon::None => None,
            MenuIcon::Checked => Some(asset_server.load("icons/Checked.png")),
            MenuIcon::Unchecked => Some(asset_server.load("icons/Unchecked.png")),
            MenuIcon::Back => Some(asset_server.load("icons/Back.png")),
            MenuIcon::Controls => Some(asset_server.load("icons/Controls.png")),
            MenuIcon::Sound => Some(asset_server.load("icons/Sound.png")),
            MenuIcon::Players => Some(asset_server.load("icons/Players.png")),
            MenuIcon::Settings => Some(asset_server.load("icons/Settings.png")),
            MenuIcon::Other(s, _) => Some(asset_server.load(s)),
        }
    }
}

#[derive(Resource)]
pub struct CustomFontData(pub Option<&'static [u8]>);

#[derive(Clone)]
pub enum WidgetText {
    PlainText(String), // FIXME: Add support for rich text
}

impl WidgetText {
    pub fn bundle(&self, default_style: &TextStyle) -> TextBundle {
        match self {
            Self::PlainText(text) => TextBundle::from_section(text, default_style.clone()),
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::PlainText(text) => text,
        }
    }
}

impl Default for WidgetText {
    fn default() -> Self {
        Self::PlainText(String::new())
    }
}

impl From<&str> for WidgetText {
    #[inline]
    fn from(text: &str) -> Self {
        Self::PlainText(text.to_string())
    }
}

impl From<&String> for WidgetText {
    #[inline]
    fn from(text: &String) -> Self {
        Self::PlainText(text.clone())
    }
}

impl From<String> for WidgetText {
    #[inline]
    fn from(text: String) -> Self {
        Self::PlainText(text)
    }
}

#[derive(Component)]
pub struct ButtonComponent<State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub style: crate::style::StyleEntry,
    pub selection: MenuSelection<A, S, State>,
    pub menu_identifier: (&'static str, usize),
    pub selected: bool,
}
