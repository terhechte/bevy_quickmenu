use std::collections::BTreeMap;

use bevy_egui::egui::{self, style::Margin, Color32, Context};

#[derive(Debug, Clone, Copy)]
pub struct ControlState {
    pub fg: Color32,
    pub bg: Option<Color32>,
    pub stroke: Color32,
    pub stroke_width: f32,
    pub rounding: f32,
}

impl ControlState {
    fn clear(fg: Color32) -> Self {
        Self {
            fg,
            bg: None,
            stroke: Color32::YELLOW,
            stroke_width: 0.0,
            rounding: 0.0,
        }
    }
    fn normal() -> Self {
        Self {
            fg: Color32::WHITE,
            bg: Some(Color32::DARK_BLUE),
            stroke: Color32::DARK_BLUE,
            stroke_width: 5.0,
            rounding: 4.0,
        }
    }

    fn hover() -> Self {
        Self {
            fg: Color32::WHITE,
            bg: Some(Color32::DARK_BLUE),
            stroke: Color32::YELLOW,
            stroke_width: 4.0,
            rounding: 4.0,
        }
    }

    fn selected() -> Self {
        Self {
            fg: Color32::BLACK,
            bg: Some(Color32::WHITE),
            stroke: Color32::YELLOW,
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
    pub size: f32,
    pub margin: Margin,
    pub padding: Margin,
    pub foreground_color: Color32,
}

impl IconStyle {
    pub fn with(size: f32, margin: Margin, padding: Margin) -> Self {
        Self {
            leading_margin: 5.0,
            trailing_margin: 10.0,
            size,
            margin,
            padding,
            foreground_color: Color32::WHITE,
        }
    }
}

impl From<&IconStyle> for Style {
    fn from(i: &IconStyle) -> Self {
        let control_state = ControlState {
            fg: i.foreground_color,
            bg: None,
            stroke: Color32::BLACK,
            stroke_width: 0.0,
            rounding: 0.0,
        };
        Style {
            size: i.size,
            margin: i.margin,
            padding: i.padding,
            normal: control_state,
            hover: control_state,
            selected: control_state,
            icon_style: IconStyle::with(i.size, i.margin, i.padding),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    pub size: f32,
    pub margin: Margin,
    pub padding: Margin,
    pub normal: ControlState,
    pub hover: ControlState,
    pub selected: ControlState,
    pub icon_style: IconStyle,
}

impl Style {
    fn button() -> Self {
        Self {
            size: 20.0,
            margin: Margin::same(5.0),
            padding: Margin::same(5.0),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
            icon_style: IconStyle::with(20.0, Margin::same(5.0), Margin::same(5.0)),
        }
    }

    fn label() -> Self {
        Self {
            size: 14.0,
            margin: Margin::same(5.0),
            padding: Margin::same(5.0),
            normal: ControlState::clear(Color32::GRAY),
            hover: ControlState::clear(Color32::GRAY),
            selected: ControlState::clear(Color32::GRAY),
            icon_style: IconStyle::with(20.0, Margin::same(5.0), Margin::same(5.0)),
        }
    }

    fn headline() -> Self {
        Self {
            size: 24.0,
            margin: Margin::same(5.0),
            padding: Margin::same(5.0),
            normal: ControlState::clear(Color32::WHITE),
            hover: ControlState::clear(Color32::WHITE),
            selected: ControlState::clear(Color32::WHITE),
            icon_style: IconStyle::with(20.0, Margin::same(5.0), Margin::same(5.0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub button: Style,
    pub label: Style,
    pub headline: Style,
    pub vertical_spacing: f32,
    pub horizontal_spacing: f32,
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self {
            button: Style::button(),
            label: Style::label(),
            headline: Style::headline(),
            vertical_spacing: 10.0,
            horizontal_spacing: 20.0,
        }
    }
}

pub fn register_stylesheet(
    stylesheet: &Stylesheet,
    context: &Context,
    font_data: Option<&'static [u8]>,
) {
    use egui::FontFamily;
    use egui::FontId;
    use egui::TextStyle::*;

    let mut text_styles = BTreeMap::new();
    text_styles.insert(
        Button,
        FontId::new(stylesheet.button.size, FontFamily::Proportional),
    );
    text_styles.insert(
        Button,
        FontId::new(stylesheet.label.size, FontFamily::Proportional),
    );
    text_styles.insert(
        Button,
        FontId::new(stylesheet.headline.size, FontFamily::Proportional),
    );

    if !text_styles.is_empty() {
        let mut style = (*context.style()).clone();
        style.text_styles = text_styles;
        context.set_style(style);
    }

    if let Some(custom_data) = font_data {
        register_font(custom_data, context);
    }
}

fn register_font(data: &'static [u8], context: &Context) {
    use bevy_egui::egui::{FontData, FontDefinitions, FontFamily};

    let mut fonts = FontDefinitions::default();

    fonts
        .font_data
        .insert("custom_font".to_owned(), FontData::from_static(data));

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "custom_font".to_owned());

    context.set_fonts(fonts);
}
