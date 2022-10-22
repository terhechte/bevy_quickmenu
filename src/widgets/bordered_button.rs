use crate::style::Style;
use bevy_egui::egui::*;

pub struct BorderedButton<'a> {
    text: &'a WidgetText,
    sense: Sense,
    min_size: Vec2,
    style: &'a Style,
    focus: bool,
}

impl<'a> BorderedButton<'a> {
    pub fn new(text: &'a WidgetText, style: &'a Style) -> Self {
        Self {
            text,
            sense: Sense::click(),
            min_size: Vec2::ZERO,
            style,
            focus: false,
        }
    }

    pub fn set_focus(mut self, value: bool) -> Self {
        self.focus = value;
        self
    }
}

impl<'a> Widget for BorderedButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let BorderedButton {
            text,
            sense,
            min_size,
            style,
            focus,
        }: BorderedButton = self;

        let total_extra = style.padding.sum() + style.margin.sum();

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text
            .clone()
            .into_galley(ui, None, wrap_width, style.text_style.clone());

        let mut desired_size = text.size() + total_extra;
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        // Focus the button automatically when it is hovered and the mouse is moving
        if response.hovered() && ui.ctx().input().pointer.velocity().length_sq() > 0.0 {
            response.request_focus();
        }

        if ui.is_rect_visible(rect) {
            let mut text_rect = rect;
            text_rect.min += style.padding.left_top() + style.margin.left_top();
            text_rect.max -= style.padding.right_bottom() + style.margin.right_bottom();
            text_rect.max.x = text_rect.max.x.max(text_rect.min.x);
            text_rect.max.y = text_rect.max.y.max(text_rect.min.y);

            let label_pos = ui
                .layout()
                .align_size_within_rect(text.size(), text_rect)
                .min;

            let controlstate = if response.is_pointer_button_down_on() {
                style.selected
            } else if response.has_focus() {
                style.hover
            } else if focus {
                style.selected
            } else {
                style.normal
            };

            let mut border_rect = rect;
            border_rect.min += style.margin.left_top();
            border_rect.max -= style.margin.right_bottom();
            border_rect.max.x = border_rect.max.x.max(border_rect.min.x);
            border_rect.max.y = border_rect.max.y.max(border_rect.min.y);

            if let Some(bg) = controlstate.bg {
                ui.painter().rect(
                    border_rect,
                    controlstate.rounding,
                    bg,
                    Stroke::new(controlstate.stroke_width, controlstate.stroke),
                );
            } else {
                ui.painter().rect_stroke(
                    rect,
                    controlstate.rounding,
                    Stroke::new(controlstate.stroke_width, controlstate.stroke),
                )
            }

            text.paint_with_fallback_color(ui.painter(), label_pos, controlstate.fg);
        }

        response
    }
}
