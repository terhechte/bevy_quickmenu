use std::collections::BTreeMap;

use bevy_egui::egui::{self, style::Margin, Color32, Context};

#[derive(Debug, Clone)]
pub struct ControlState {
    pub fg: Color32,
    pub bg: Color32,
    pub stroke: Color32,
    pub stroke_width: f32,
}

impl ControlState {
    fn normal() -> Self {
        Self {
            fg: Color32::WHITE,
            bg: Color32::DARK_BLUE,
            stroke: Color32::DARK_BLUE,
            stroke_width: 2.0,
        }
    }

    fn hover() -> Self {
        Self {
            fg: Color32::WHITE,
            bg: Color32::DARK_BLUE,
            stroke: Color32::YELLOW,
            stroke_width: 2.0,
        }
    }

    fn selected() -> Self {
        Self {
            fg: Color32::BLACK,
            bg: Color32::YELLOW,
            stroke: Color32::YELLOW,
            stroke_width: 2.0,
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
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: 20.0,
            margin: Default::default(),
            padding: Default::default(),
            normal: ControlState::normal(),
            hover: ControlState::hover(),
            selected: ControlState::selected(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Stylesheet {
    pub back_button: Option<Style>,
    pub button: Option<Style>,
    pub label: Option<Style>,
    pub headline: Option<Style>,
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
    if let Some(ref button) = stylesheet.button {
        text_styles.insert(Button, FontId::new(button.size, FontFamily::Proportional));
    }
    if let Some(ref label) = stylesheet.label {
        text_styles.insert(Body, FontId::new(label.size, FontFamily::Proportional));
    }
    if let Some(ref headline) = stylesheet.headline {
        text_styles.insert(
            Heading,
            FontId::new(headline.size, FontFamily::Proportional),
        );
    }

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
