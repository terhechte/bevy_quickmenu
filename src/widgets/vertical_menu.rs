use crate::{
    style::{Style, Stylesheet},
    types::{CursorDirection, MenuIcon, MenuItem, MenuSelection},
    ActionTrait, ScreenTrait,
};
use bevy_egui::egui::{Id, Response, Sense, Ui, Vec2, Widget};

use super::{BorderedButton, BorderedLabel};

pub struct VerticalMenu<'a, State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    // Each menu needs a distinct id
    pub id: Id,
    // The last cursor direction we saw
    pub cursor_direction: Option<CursorDirection>,
    // The items in the menu
    pub items: &'a [MenuItem<State, A, S>],
    // selection
    pub selection: &'a mut Option<MenuSelection<A, S, State>>,
    // stylesheet
    pub stylesheet: &'a Stylesheet,
}

impl<'a, State, A, S> Widget for VerticalMenu<'a, State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let VerticalMenu {
            id,
            cursor_direction,
            items,
            selection,
            stylesheet,
        } = self;
        if items.is_empty() {
            return ui.allocate_response(Vec2::ZERO, Sense::click());
        }
        ui.vertical(|ui| {
            let mut selected = ui
                .memory()
                .data
                .get_temp::<MenuSelection<A, S, State>>(id)
                .unwrap_or_else(|| {
                    // First, try to find the first non-None selection,
                    // otherwise, take the first
                    let non_none = items
                        .iter()
                        .find(|e| e.as_selection() != MenuSelection::None)
                        .map(|e| e.as_selection());
                    non_none.unwrap_or_else(|| items.first().map(MenuItem::as_selection).unwrap())
                });

            let mut select_navigation = false;

            if let (Some(cursor_direction), Some(mut index)) = (
                cursor_direction,
                items.iter().position(|e| e.as_selection() == selected),
            ) {
                loop {
                    match cursor_direction {
                        CursorDirection::Up if index > 0 => index -= 1,
                        CursorDirection::Down if index < (items.len() - 1) => index += 1,
                        CursorDirection::Select => select_navigation = true,
                        _ => (),
                    }
                    if !matches!(items[index], MenuItem::Label(_, _)) {
                        break;
                    }
                }
                // change selection
                selected = items[index].as_selection();
            }

            for item in items {
                let is_label = matches!(item, MenuItem::Label(_, _));
                let item_selection = item.as_selection();
                let focussed = (selected == item_selection) && !is_label;
                let response = match item {
                    MenuItem::Screen(t, i, _) => add_item(
                        ui,
                        i,
                        &stylesheet.button,
                        BorderedButton::new(t, &stylesheet.button).set_focus(focussed),
                    ),
                    MenuItem::Action(t, i, _) => add_item(
                        ui,
                        i,
                        &stylesheet.button,
                        BorderedButton::new(t, &stylesheet.button).set_focus(focussed),
                    ),
                    MenuItem::Label(t, i) => add_item(
                        ui,
                        i,
                        &stylesheet.label,
                        BorderedLabel::new(t, &stylesheet.label),
                    ),
                    MenuItem::Headline(t, i) => add_item(
                        ui,
                        i,
                        &stylesheet.headline,
                        BorderedLabel::new(t, &stylesheet.headline),
                    ),
                };
                if !is_label && (response.clicked() || (select_navigation && focussed)) {
                    selected = item_selection.clone();
                    *selection = Some(item_selection);
                }
                ui.add_space(stylesheet.vertical_spacing);
            }

            ui.memory().data.insert_temp(id, selected);

            ui.allocate_response(Vec2::ZERO, Sense::click())
        })
        .inner
    }
}

fn add_item(ui: &mut Ui, icon: &MenuIcon, style: &Style, widget: impl Widget) -> Response {
    let icon_style: Style = style.as_iconstyle();
    let is_postfix = icon.is_postfix();
    ui.horizontal(|ui| {
        if !is_postfix {
            if let Some(t) = icon.icon() {
                ui.add(BorderedLabel::new(&t.into(), &icon_style));
                ui.add_space(style.icon_style.leading_margin);
            }
        }
        let response = ui.add(widget);
        if is_postfix {
            if let Some(t) = icon.icon() {
                ui.add_space(style.icon_style.trailing_margin);
                ui.add(BorderedLabel::new(&t.into(), &icon_style));
            }
        }
        response
    })
    .inner
}
