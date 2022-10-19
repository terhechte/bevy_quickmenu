use bevy_egui::egui::{style::Margin, Color32};

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
    pub size: usize,
    pub margin: Margin,
    pub padding: Margin,
    pub normal: ControlState,
    pub hover: ControlState,
    pub selected: ControlState,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            size: Default::default(),
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
}
