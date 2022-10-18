use bevy_egui::egui::{self, *};

pub struct BorderedButton {
    text: WidgetText,
    sense: Sense,
    min_size: Vec2,
    default_border: Option<Stroke>,
    on_focus_border: Option<Stroke>,
    on_click_border: Option<Stroke>,
    margin: egui::style::Margin,
    padding: egui::style::Margin,
    focussed: bool,
}

impl BorderedButton {
    // Create a new button
    #[must_use = "You must call .show() to render the button"]
    pub fn new(text: WidgetText) -> Self {
        Self {
            text,
            sense: Sense::click(),
            min_size: Vec2::ZERO,
            default_border: None,
            on_focus_border: None,
            on_click_border: None,
            margin: Default::default(),
            padding: Default::default(),
            focussed: false,
        }
    }

    pub fn focussed(mut self, value: bool) -> Self {
        self.focussed = value;
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        self.ui(ui)
    }
}

impl Widget for BorderedButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let BorderedButton {
            text,
            sense,
            min_size,
            default_border,
            on_focus_border,
            on_click_border,
            margin,
            padding,
            focussed,
        }: BorderedButton = self;

        let total_extra = padding.sum() + margin.sum();

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = text.size() + total_extra;
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        // Focus the button automatically when it is hovered and the mouse is moving
        if response.hovered() && ui.ctx().input().pointer.velocity().length_sq() > 0.0 {
            response.request_focus();
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let mut text_rect = rect;
            text_rect.min += padding.left_top() + margin.left_top();
            text_rect.max -= padding.right_bottom() + margin.right_bottom();
            text_rect.max.x = text_rect.max.x.max(text_rect.min.x);
            text_rect.max.y = text_rect.max.y.max(text_rect.min.y);

            let label_pos = ui
                .layout()
                .align_size_within_rect(text.size(), text_rect)
                .min;

            let border = if response.is_pointer_button_down_on() {
                on_click_border.or(default_border)
            } else if response.has_focus() {
                on_focus_border.or(default_border)
            } else {
                default_border
            };

            let mut border_rect = rect;
            border_rect.min += margin.left_top();
            border_rect.max -= margin.right_bottom();
            border_rect.max.x = border_rect.max.x.max(border_rect.min.x);
            border_rect.max.y = border_rect.max.y.max(border_rect.min.y);

            if let Some(border) = border {
                ui.painter()
                    .rect(border_rect, 0.0, Color32::DARK_RED, border);
            }

            if focussed {
                ui.painter()
                    .rect(border_rect, 0.0, Color32::BLUE, Stroke::default());
            }

            text.paint_with_visuals(ui.painter(), label_pos, visuals);
        }

        response
    }
}
