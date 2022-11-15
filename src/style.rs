use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ControlState {
    pub fg: Color,
    pub bg: Color,
}

impl ControlState {
    fn clear(fg: Color) -> Self {
        Self {
            fg,
            bg: Color::rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
    fn normal() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::BLUE,
        }
    }

    fn hover() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::RED,
        }
    }

    fn selected() -> Self {
        Self {
            fg: Color::BLACK,
            bg: Color::WHITE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconStyle {
    /// The size of the icon
    pub size: Size,
    /// The padding
    pub padding: UiRect,
    /// An alternative foreground color
    pub tint_color: Color,
}

impl Default for IconStyle {
    fn default() -> Self {
        Self {
            size: Size::new(Val::Px(32.0), Val::Px(32.0)),
            padding: UiRect::all(Val::Px(14.0)),
            tint_color: Color::YELLOW,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleEntry {
    pub size: f32,
    pub font: Handle<Font>,
    pub margin: UiRect,
    pub padding: UiRect,
    pub normal: ControlState,
    pub hover: ControlState,
    pub selected: ControlState,
    pub icon_style: IconStyle,
}

impl StyleEntry {
    fn button() -> Self {
        Self {
            size: 20.0,
            font: Default::default(),
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
            icon_style: IconStyle::default(),
        }
    }

    fn label() -> Self {
        Self {
            size: 20.0, // FIXME: Was 14
            font: Default::default(),
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::clear(Color::WHITE),
            hover: ControlState::clear(Color::WHITE),
            selected: ControlState::clear(Color::WHITE),
            icon_style: IconStyle::default(),
        }
    }

    fn headline() -> Self {
        Self {
            size: 24.0,
            font: Default::default(),
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
    pub horizontal_spacing: f32,
}

impl Stylesheet {
    pub fn with_font(font: Handle<Font>) -> Self {
        let mut sheet = Self::default();
        sheet.button.font = font.clone();
        sheet.label.font = font.clone();
        sheet.headline.font = font;
        sheet
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self {
            button: StyleEntry::button(),
            label: StyleEntry::label(),
            headline: StyleEntry::headline(),
            vertical_spacing: 10.0,
            horizontal_spacing: 20.0,
        }
    }
}
