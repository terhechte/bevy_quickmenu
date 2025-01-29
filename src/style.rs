//! Lightweight abstractions over styles
//! Instead of using the bevy styles with all their properties, these simplified
//! styles are mostly used to define the looks of menus and the different
//! control states of buttons.

use bevy::{prelude::*, text::FontSmoothing};

#[derive(Debug, Clone, Copy)]
pub struct ControlState {
    pub fg: Color,
    pub bg: Color,
}

impl ControlState {
    fn clear(fg: Color) -> Self {
        Self {
            fg,
            bg: Color::srgba(0.0, 0.0, 0.0, 0.0),
        }
    }
    fn normal() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::srgb(0.0, 0.0, 0.5),
        }
    }

    fn hover() -> Self {
        Self {
            fg: Color::srgb(1.0, 1.0, 0.0),
            bg: Color::srgb(0.0, 0.0, 0.5),
        }
    }

    fn selected() -> Self {
        Self {
            fg: Color::srgb(0.0, 0.0, 0.5),
            bg: Color::WHITE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconStyle {
    /// The width of the icon
    pub width: Val,
    /// The size of the icon
    pub height: Val,
    /// The padding
    pub padding: UiRect,
    /// An alternative foreground color
    pub tint_color: Color,
}

impl Default for IconStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            padding: UiRect::all(Val::Px(6.0)),
            tint_color: Color::WHITE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleEntry {
    pub size: f32,
    pub smoothing: FontSmoothing,
    pub margin: UiRect,
    pub padding: UiRect,
    pub normal: ControlState,
    pub hover: ControlState,
    pub selected: ControlState,
    pub icon_style: IconStyle,
}

impl StyleEntry {
    pub fn button() -> Self {
        Self {
            size: 20.0,
            smoothing: FontSmoothing::AntiAliased,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
            icon_style: IconStyle::default(),
        }
    }

    pub fn label() -> Self {
        let gray = Color::srgb(0.7, 0.7, 0.7);
        Self {
            size: 18.0,
            smoothing: FontSmoothing::AntiAliased,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::clear(gray),
            hover: ControlState::clear(gray),
            selected: ControlState::clear(gray),
            icon_style: IconStyle::default(),
        }
    }

    pub fn headline() -> Self {
        Self {
            size: 24.0,            
            smoothing: FontSmoothing::AntiAliased,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::clear(Color::WHITE),
            hover: ControlState::clear(Color::WHITE),
            selected: ControlState::clear(Color::WHITE),
            icon_style: IconStyle::default(),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct Stylesheet {
    pub button: StyleEntry,
    pub label: StyleEntry,
    pub headline: StyleEntry,
    pub vertical_spacing: f32,
    pub style: Option<Node>,
    pub background: Option<BackgroundColor>,
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self {
            button: StyleEntry::button(),
            label: StyleEntry::label(),
            headline: StyleEntry::headline(),
            vertical_spacing: 10.0,
            style: None,
            background: None,
        }
    }
}

impl Stylesheet {
    pub fn with_background(mut self, bg: BackgroundColor) -> Self {
        self.background = Some(bg);
        self
    }

    pub fn with_style(mut self, style: Node) -> Self {
        self.style = Some(style);
        self
    }
}
