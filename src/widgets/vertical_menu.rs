use crate::{
    style::Stylesheet,
    types::{CursorDirection, MenuItem, MenuSelection},
    ActionTrait, ScreenTrait,
};
use bevy_egui::egui::{Id, Response, Sense, Ui, Vec2, Widget};

use super::BorderedButton;

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
    // pub stylesheet: &'a Stylesheet,
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
            // stylesheet,
        } = self;
        if items.is_empty() {
            return ui.allocate_response(Vec2::ZERO, Sense::click());
        }
        ui.vertical(|ui| {
            let mut selected = ui
                .memory()
                .data
                .get_temp::<MenuSelection<A, S, State>>(id)
                .unwrap_or_else(|| items.first().map(MenuItem::as_selection).unwrap());

            let mut select_navigation = false;

            if let (Some(cursor_direction), Some(mut index)) = (
                cursor_direction,
                items.iter().position(|e| e.as_selection() == selected),
            ) {
                match cursor_direction {
                    CursorDirection::Up if index > 0 => index -= 1,
                    CursorDirection::Down if index < (items.len() - 1) => index += 1,
                    CursorDirection::Select => select_navigation = true,
                    _ => (),
                }
                // change selection
                selected = items[index].as_selection();
            }

            for item in items {
                let item_selection = item.as_selection();
                let focussed = selected == item_selection;
                let response =
                    ui.add(BorderedButton::new(item.text().clone(), None).set_focus(focussed));
                if response.clicked() || (select_navigation && focussed) {
                    selected = item_selection.clone();
                    *selection = Some(item_selection);
                }
            }

            ui.memory().data.insert_temp(id, selected);

            ui.allocate_response(Vec2::ZERO, Sense::click())
        })
        .inner
    }
}
