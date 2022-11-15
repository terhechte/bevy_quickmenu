use crate::style::StyleEntry;
use crate::types::{ButtonComponent, WidgetText};
use crate::{ActionTrait, MenuSelection, ScreenTrait};
use bevy::prelude::*;
// use bevy_egui::egui::*;
use super::Widget;

pub struct BorderedLabel<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    text: &'a WidgetText,
    min_size: Vec2,
    style: &'a StyleEntry,
    selection: &'a MenuSelection<A, S, State>,
}

impl<'a, A, S, State> BorderedLabel<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn new(
        text: &'a WidgetText,
        style: &'a StyleEntry,
        selection: &'a MenuSelection<A, S, State>,
    ) -> Self {
        Self {
            text,
            min_size: Vec2::ZERO,
            style,
            selection,
        }
    }
}

impl<'a, A, S, State> Widget for BorderedLabel<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    type A = A;
    type S = S;
    type State = State;
    fn build(
        self,
        parent: &mut ChildBuilder,
        menu_identifier: (&'static str, usize),
        selected: bool,
        active: bool,
    ) {
        let BorderedLabel {
            text,
            min_size,
            style,
            selection,
        } = self;

        // let text_style = TextStyle {
        //     // font: fonts.bold.clone(),
        //     font_size: 30.0,
        //     color: Color::RED,
        //     ..Default::default()
        // };

        let background_color = if selected {
            style.selected.bg
        } else {
            style.normal.bg
        }
        .unwrap_or(Color::BLACK); // FIXME: TRANSPARENT!

        parent
            .spawn(ButtonBundle {
                style: Style {
                    // size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(background_color),
                ..default()
            })
            .insert(ButtonComponent {
                style: style.clone(),
                selection: selection.clone(),
                menu_identifier,
            })
            .with_children(|parent| {
                parent.spawn(text.bundle(&self.style.text_style));
            });

        // let total_extra = style.padding.sum() + style.margin.sum();

        // let wrap_width = ui.available_width() - total_extra.x;
        // let text = text
        //     .clone()
        //     .into_galley(ui, None, wrap_width, style.text_style.clone());

        // let mut desired_size = text.size() + total_extra;
        // desired_size = desired_size.at_least(min_size);

        // let (rect, response) =
        //     ui.allocate_at_least(desired_size, Sense::focusable_noninteractive());
        // response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, text.text()));

        // // Focus the button automatically when it is hovered and the mouse is moving
        // if response.hovered() && ui.ctx().input().pointer.velocity().length_sq() > 0.0 {
        //     response.request_focus();
        // }

        // if ui.is_rect_visible(rect) {
        //     let mut text_rect = rect;
        //     text_rect.min += style.padding.left_top() + style.margin.left_top();
        //     text_rect.max -= style.padding.right_bottom() + style.margin.right_bottom();
        //     text_rect.max.x = text_rect.max.x.max(text_rect.min.x);
        //     text_rect.max.y = text_rect.max.y.max(text_rect.min.y);

        //     let label_pos = ui
        //         .layout()
        //         .align_size_within_rect(text.size(), text_rect)
        //         .min;

        //     let controlstate = style.normal;

        //     let mut border_rect = rect;
        //     border_rect.min += style.margin.left_top();
        //     border_rect.max -= style.margin.right_bottom();
        //     border_rect.max.x = border_rect.max.x.max(border_rect.min.x);
        //     border_rect.max.y = border_rect.max.y.max(border_rect.min.y);

        //     if let Some(bg) = controlstate.bg {
        //         ui.painter().rect(
        //             border_rect,
        //             controlstate.rounding,
        //             bg,
        //             Stroke::new(controlstate.stroke_width, controlstate.stroke),
        //         );
        //     } else {
        //         ui.painter().rect_stroke(
        //             rect,
        //             controlstate.rounding,
        //             Stroke::new(controlstate.stroke_width, controlstate.stroke),
        //         )
        //     }

        //     text.paint_with_fallback_color(ui.painter(), label_pos, controlstate.fg);
        // }

        // response
    }
}
