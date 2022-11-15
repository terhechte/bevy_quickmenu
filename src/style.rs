use std::collections::BTreeMap;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ControlState {
    pub fg: Color,
    pub bg: Option<Color>,
    pub stroke: Color,
    pub stroke_width: f32,
    pub rounding: f32,
}

impl ControlState {
    fn clear(fg: Color) -> Self {
        Self {
            fg,
            bg: None,
            stroke: Color::YELLOW,
            stroke_width: 0.0,
            rounding: 0.0,
        }
    }
    fn normal() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Some(Color::BLUE),
            stroke: Color::BLUE,
            stroke_width: 5.0,
            rounding: 4.0,
        }
    }

    fn hover() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Some(Color::BLUE),
            stroke: Color::YELLOW,
            stroke_width: 4.0,
            rounding: 4.0,
        }
    }

    fn selected() -> Self {
        Self {
            fg: Color::BLACK,
            bg: Some(Color::WHITE),
            stroke: Color::YELLOW,
            stroke_width: 3.0,
            rounding: 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconStyle {
    /// The leading margin is only used for prefix icons
    pub leading_margin: f32,
    /// The trailing margin is only used for postfix icons
    pub trailing_margin: f32,
    /// An alternative foreground color
    pub foreground_color: Color,
}

impl Default for IconStyle {
    fn default() -> Self {
        Self {
            leading_margin: 5.0,
            trailing_margin: 10.0,
            foreground_color: Color::WHITE,
        }
    }
}

impl StyleEntry {
    /// Create a new style with the adaptions for the icons for this element
    pub fn as_iconstyle(&self) -> StyleEntry {
        let control_state = ControlState {
            fg: self.icon_style.foreground_color,
            bg: None,
            stroke: Color::BLACK,
            stroke_width: 0.0,
            rounding: 0.0,
        };
        StyleEntry {
            size: self.size,
            margin: self.margin,
            padding: self.padding,
            normal: control_state,
            hover: control_state,
            selected: control_state,
            icon_style: self.icon_style.clone(),
            text_style: self.text_style.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleEntry {
    pub size: f32,
    pub margin: UiRect,
    pub padding: UiRect,
    pub normal: ControlState,
    pub hover: ControlState,
    pub selected: ControlState,
    pub icon_style: IconStyle,
    pub text_style: TextStyle,
}

impl StyleEntry {
    fn button() -> Self {
        Self {
            size: 20.0,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
            icon_style: IconStyle::default(),
            text_style: TextStyle::default(),
        }
    }

    fn label() -> Self {
        Self {
            size: 14.0,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            // FIXME: RESET TO BACK
            // normal: ControlState::clear(Color::GRAY),
            // hover: ControlState::clear(Color::GRAY),
            // selected: ControlState::clear(Color::GRAY),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
            icon_style: IconStyle::default(),
            text_style: TextStyle::default(),
        }
    }

    fn headline() -> Self {
        Self {
            size: 24.0,
            margin: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            normal: ControlState::clear(Color::WHITE),
            hover: ControlState::clear(Color::WHITE),
            selected: ControlState::clear(Color::WHITE),
            icon_style: IconStyle::default(),
            text_style: TextStyle::default(),
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
        sheet.button.text_style.font = font.clone();
        sheet.label.text_style.font = font.clone();
        sheet.headline.text_style.font = font.clone();
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
