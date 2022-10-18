use bevy::prelude::EventWriter;
use bevy_egui::egui::{self, *};
use std::fmt::Debug;
use std::hash::Hash;

pub trait ActionTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type State;
    fn handle(&self, state: &mut Self::State);
}

pub trait ScreenTrait: Debug + PartialEq + Eq + Clone + Copy + Hash + Send + Sync {
    type Action: ActionTrait;
    fn resolve(
        &self,
        ui: &mut Ui,
        state: &mut <<Self as ScreenTrait>::Action as ActionTrait>::State,
        cursor_direction: Option<CursorDirection>,
    ) -> Option<
        MenuSelection<Self::Action, Self, <<Self as ScreenTrait>::Action as ActionTrait>::State>,
    >;
}

#[derive(Debug)]
pub enum MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Action(A),
    Screen(S),
}

impl<A, S, State> Clone for MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Action(arg0) => Self::Action(*arg0),
            Self::Screen(arg0) => Self::Screen(*arg0),
        }
    }
}

impl<A, S, State> PartialEq for MenuSelection<A, S, State>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MenuSelection::Action(a1), MenuSelection::Action(a2)) => a1 == a2,
            (MenuSelection::Screen(s1), MenuSelection::Screen(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    /// The internal stack of menu screens
    stack: Vec<S>,
    /// Any user input (cursor keys, gamepad) is set here
    next_direction: Option<CursorDirection>,
    /// The custom state
    state: State,
}

impl<State, A, S> NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn new(state: State, root: S) -> Self {
        Self {
            stack: vec![root],
            next_direction: None,
            state,
        }
    }

    pub fn next(&mut self, direction: CursorDirection) {
        self.next_direction = Some(direction);
    }
}

impl<State, A, S> Widget for &mut NavigationMenu<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let next_direction = self.next_direction.take();

        if let Some(CursorDirection::Back) = next_direction {
            self.stack.pop();
        }

        let mut next_menu: Option<MenuSelection<A, S, State>> = None;

        let response = ui.horizontal(|ui| {
            for (index, entry) in self.stack.iter().enumerate() {
                let is_last = (index + 1) == self.stack.len();
                let cursor_direction = if is_last { next_direction } else { None };
                if let Some(next) = entry.resolve(ui, &mut self.state, cursor_direction) {
                    if is_last {
                        next_menu = Some(next);
                    }
                }
            }
            ui.allocate_response(Vec2::ZERO, Sense::click())
        });

        if let Some(n) = next_menu {
            match n {
                MenuSelection::Action(a) => a.handle(&mut self.state),
                MenuSelection::Screen(s) => self.stack.push(s),
            }
        }

        response.inner
    }
}

pub fn make_menu<State, A, S>(
    ui: &mut Ui,
    id: Id,
    cursor_direction: Option<CursorDirection>,
    items: Vec<MenuItem<State, A, S>>,
) -> Option<MenuSelection<A, S, State>>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    let mut selection: Option<MenuSelection<A, S, State>> = None;
    ui.add(VerticalMenu {
        id,
        cursor_direction,
        items,
        selection: &mut selection,
    });
    selection
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorDirection {
    Up,
    Down,
    Select,
    Back,
}

pub enum MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    Screen(WidgetText, S),
    Action(WidgetText, A),
}

impl<State, A, S> MenuItem<State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    pub fn screen(s: impl Into<WidgetText>, screen: S) -> Self {
        MenuItem::Screen(s.into(), screen)
    }

    pub fn action(s: impl Into<WidgetText>, action: A) -> Self {
        MenuItem::Action(s.into(), action)
    }

    fn as_selection(&self) -> MenuSelection<A, S, State> {
        match self {
            MenuItem::Screen(_, a) => MenuSelection::Screen(*a),
            MenuItem::Action(_, a) => MenuSelection::Action(*a),
        }
    }

    fn text(&self) -> &WidgetText {
        match self {
            MenuItem::Screen(t, _) => t,
            MenuItem::Action(t, _) => t,
        }
    }
}

struct VerticalMenu<'a, State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    // Each menu needs a distinct id
    id: Id,
    // The last cursor direction we saw
    cursor_direction: Option<CursorDirection>,
    // The items in the menu
    items: Vec<MenuItem<State, A, S>>,
    // selection
    selection: &'a mut Option<MenuSelection<A, S, State>>,
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
                let response = ui.add(BorderedButton::new(item.text().clone()).focussed(focussed));
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

// https://github.com/fishfolks/punchy/blob/master/src/ui/widgets/bordered_button.rs

/// Bordered button rendering
///
/// Adapted from <https://docs.rs/egui/0.18.1/src/egui/widgets/button.rs.html>

/// A button rendered with a [`BorderImage`]
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
