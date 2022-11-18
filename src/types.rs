use crate::{ActionTrait, ScreenTrait};
use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::utils::HashMap;

#[derive(Component)]
pub struct QuickMenuComponent;

/// The primary horizontal menu can be queried via this component
#[derive(Component)]
pub struct PrimaryMenu;

/// Each vertical menu can be queried via this component
#[derive(Component)]
pub struct VerticalMenuComponent(pub &'static str);

/// Each Button in the UI can be queried via this component in order
/// to further change the appearance
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

/// Helper to remove the Menu. This `Resource` is inserted to notify
/// the `cleanup_system` that the menu can be removed.
#[derive(Resource, Default)]
pub struct CleanUpUI;

/// This map holds the currently selected items in each screen / menu
#[derive(Resource, Default)]
pub struct Selections(pub HashMap<&'static str, usize>);

/// GamePad and Cursor navigation generates these navigation events
/// which are then processed by a system and applied to the menu.
/// Navigation can be customized by sending these events into a
/// `EventWriter<NavigationEvent>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationEvent {
    Up,
    Down,
    Select,
    Back,
}

/// Whenever a state change in the `MenuState` is detected,
/// this event is send in order to tell the UI to re-render itself
pub struct RedrawEvent;

/// Create a menu with an identifier and a `Vec` of `MenuItem` entries
pub struct Menu<A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub id: &'static str,
    pub entries: Vec<MenuItem<State, A, S>>,
    pub style: Option<Style>,
    pub background: Option<BackgroundColor>,
}

impl<A, S, State> Menu<A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn new(id: &'static str, entries: Vec<MenuItem<State, A, S>>) -> Self {
        Self {
            id,
            entries,
            style: None,
            background: None,
        }
    }

    pub fn with_background(mut self, bg: BackgroundColor) -> Self {
        self.background = Some(bg);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// Abstraction over MenuItems in a Screen / Menu
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
            Self::Screen(arg0, _, _) => f.debug_tuple("Screen").field(&arg0.debug_text()).finish(),
            Self::Action(arg0, _, _) => f.debug_tuple("Action").field(&arg0.debug_text()).finish(),
            Self::Label(arg0, _) => f.debug_tuple("Label").field(&arg0.debug_text()).finish(),
            Self::Headline(arg0, _) => f.debug_tuple("Headline").field(&arg0.debug_text()).finish(),
        }
    }
}

/// Abstraction over a concrete selection in a screen / menu
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

/// The library comes with some pre-defined icons for several screens.
/// Custom icons can be used with `MenuIcon::Other` or by overriding
/// the existing ones via `MenuOptions`
pub enum MenuIcon {
    None,
    Checked,
    Unchecked,
    Back,
    Controls,
    Sound,
    Players,
    Settings,
    Other(Handle<Image>),
}

impl MenuIcon {
    pub(crate) fn resolve_icon(&self, assets: &MenuAssets) -> Option<Handle<Image>> {
        match self {
            MenuIcon::None => None,
            MenuIcon::Checked => Some(assets.icon_checked.clone()),
            MenuIcon::Unchecked => Some(assets.icon_unchecked.clone()),
            MenuIcon::Back => Some(assets.icon_back.clone()),
            MenuIcon::Controls => Some(assets.icon_controls.clone()),
            MenuIcon::Sound => Some(assets.icon_sound.clone()),
            MenuIcon::Players => Some(assets.icon_players.clone()),
            MenuIcon::Settings => Some(assets.icon_settings.clone()),
            MenuIcon::Other(s) => Some(s.clone()),
        }
    }
}

/// Simplified Rich-Text that assumes the default font
#[derive(Clone, Debug, Default)]
pub struct RichTextEntry {
    pub text: String,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub font: Option<Handle<Font>>,
}

impl RichTextEntry {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            text: text.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn new_color(text: impl AsRef<str>, color: Color) -> Self {
        Self {
            text: text.as_ref().to_string(),
            color: Some(color),
            ..Default::default()
        }
    }
}

/// Abstraction over text for buttons and labels
#[derive(Clone, Debug)]
pub enum WidgetText {
    PlainText(String),
    RichText(Vec<RichTextEntry>),
}

impl WidgetText {
    pub fn bundle(&self, default_style: &TextStyle) -> TextBundle {
        match self {
            Self::PlainText(text) => TextBundle::from_section(text, default_style.clone()),
            Self::RichText(entries) => TextBundle::from_sections(entries.iter().map(|entry| {
                TextSection {
                    value: entry.text.clone(),
                    style: TextStyle {
                        font: entry
                            .font
                            .as_ref()
                            .cloned()
                            .unwrap_or_else(|| default_style.font.clone()),
                        font_size: entry.size.unwrap_or(default_style.font_size),
                        color: entry.color.unwrap_or(default_style.color),
                    },
                }
            })),
        }
    }

    pub fn debug_text(&self) -> String {
        match self {
            Self::PlainText(text) => text.clone(),
            Self::RichText(entries) => {
                let mut output = String::new();
                for entry in entries {
                    output.push_str(&entry.text);
                    output.push(' ');
                }
                output
            }
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

impl<const N: usize> From<[RichTextEntry; N]> for WidgetText {
    #[inline]
    fn from(rich: [RichTextEntry; N]) -> Self {
        Self::RichText(rich.to_vec())
    }
}

/// Changing these `MenuOptions` allows overriding the provided
/// images and fonts. Use [`QUickMenuPlugin::with_options`] to do this.
#[derive(Resource, Default, Clone, Copy)]
pub struct MenuOptions {
    pub font: Option<&'static str>,
    pub icon_checked: Option<&'static str>,
    pub icon_unchecked: Option<&'static str>,
    pub icon_back: Option<&'static str>,
    pub icon_controls: Option<&'static str>,
    pub icon_sound: Option<&'static str>,
    pub icon_players: Option<&'static str>,
    pub icon_settings: Option<&'static str>,
}

#[derive(Resource)]
pub struct MenuAssets {
    pub font: Handle<Font>,
    pub icon_checked: Handle<Image>,
    pub icon_unchecked: Handle<Image>,
    pub icon_back: Handle<Image>,
    pub icon_controls: Handle<Image>,
    pub icon_sound: Handle<Image>,
    pub icon_players: Handle<Image>,
    pub icon_settings: Handle<Image>,
}

impl FromWorld for MenuAssets {
    fn from_world(world: &mut World) -> Self {
        let options = *(world.get_resource::<MenuOptions>().unwrap());
        let font = {
            let assets = world.get_resource::<AssetServer>().unwrap();
            let font = match options.font {
                Some(font) => assets.load(font),
                None => world.get_resource_mut::<Assets<Font>>().unwrap().add(
                    Font::try_from_bytes(include_bytes!("default_font.ttf").to_vec()).unwrap(),
                ),
            };
            font
        };
        fn load_icon(
            alt: Option<&'static str>,
            else_bytes: &'static [u8],
            world: &mut World,
        ) -> Handle<Image> {
            let assets = world.get_resource::<AssetServer>().unwrap();
            match alt {
                Some(image) => assets.load(image),
                None => world.get_resource_mut::<Assets<Image>>().unwrap().add(
                    Image::from_buffer(
                        else_bytes,
                        ImageType::Extension("png"),
                        CompressedImageFormats::empty(),
                        true,
                    )
                    .unwrap(),
                ),
            }
        }

        let icon_unchecked = load_icon(
            options.icon_unchecked,
            include_bytes!("default_icons/Unchecked.png"),
            world,
        );

        let icon_checked = load_icon(
            options.icon_checked,
            include_bytes!("default_icons/Checked.png"),
            world,
        );

        let icon_back = load_icon(
            options.icon_back,
            include_bytes!("default_icons/Back.png"),
            world,
        );

        let icon_controls = load_icon(
            options.icon_controls,
            include_bytes!("default_icons/Controls.png"),
            world,
        );

        let icon_sound = load_icon(
            options.icon_sound,
            include_bytes!("default_icons/Sound.png"),
            world,
        );

        let icon_players = load_icon(
            options.icon_players,
            include_bytes!("default_icons/Players.png"),
            world,
        );

        let icon_settings = load_icon(
            options.icon_settings,
            include_bytes!("default_icons/Settings.png"),
            world,
        );

        Self {
            font,
            icon_checked,
            icon_unchecked,
            icon_back,
            icon_controls,
            icon_sound,
            icon_players,
            icon_settings,
        }
    }
}
