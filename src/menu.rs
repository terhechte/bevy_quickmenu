use bevy_egui::egui::{self, *};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MenuSelection {
    Action(Actions),
    Screen(Screens),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ControlDevice {
    Keyboard(usize),
    Gamepad(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Close,
    SoundOn,
    SoundOff,
    Control(usize, ControlDevice),
}

impl Actions {
    fn handle<State>(&self, state: &mut State) {
        match self {
            Actions::Close => return,
            Actions::SoundOn => return,
            Actions::SoundOff => return,
            Actions::Control(_, _) => return,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Players,
    Controls,
    Sound,
    Player(usize),
}

impl Screens {
    fn resolve<State>(
        &self,
        ui: &mut Ui,
        state: &mut State,
        cursor_direction: Option<CursorDirection>,
    ) -> Option<MenuSelection> {
        match self {
            Screens::Root => root_menu(ui, cursor_direction),
            Screens::Players => sound_menu(ui, cursor_direction),
            Screens::Controls => sound_menu(ui, cursor_direction),
            Screens::Sound => sound_menu(ui, cursor_direction),
            Screens::Player(_) => sound_menu(ui, cursor_direction),
        }
    }
}

#[derive(Debug)]
pub struct NavigationMenu<State: std::fmt::Debug> {
    /// The internal stack of menu screens
    stack: Vec<Screens>,
    /// Any user input (cursor keys, gamepad) is set here
    next_direction: Option<CursorDirection>,
    /// The custom state
    state: State,
}

impl<State: std::fmt::Debug> NavigationMenu<State> {
    pub fn new(state: State) -> Self {
        Self {
            stack: vec![Screens::Root],
            next_direction: None,
            state,
        }
    }
}

impl<State: std::fmt::Debug> NavigationMenu<State> {
    pub fn next(&mut self, direction: CursorDirection) {
        self.next_direction = Some(direction);
    }
}

impl<State: std::fmt::Debug> Widget for &mut NavigationMenu<State> {
    fn ui(self, ui: &mut Ui) -> Response {
        let next_direction = self.next_direction.take();

        if let Some(CursorDirection::Back) = next_direction {
            self.stack.pop();
        }

        let mut next_menu: Option<MenuSelection> = None;

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

fn root_menu(ui: &mut Ui, cursor_direction: Option<CursorDirection>) -> Option<MenuSelection> {
    make_menu(
        ui,
        Id::new("root"),
        cursor_direction,
        vec![
            MenuItem::action("Back", Actions::Close),
            MenuItem::screen("Sound", Screens::Sound),
            MenuItem::screen("Controls", Screens::Controls),
        ],
    )
}

fn sound_menu(ui: &mut Ui, cursor_direction: Option<CursorDirection>) -> Option<MenuSelection> {
    make_menu(
        ui,
        Id::new("sound"),
        cursor_direction,
        vec![
            MenuItem::action("On", Actions::SoundOn),
            MenuItem::action("Off", Actions::SoundOff),
        ],
    )
}

fn make_menu(
    ui: &mut Ui,
    id: Id,
    cursor_direction: Option<CursorDirection>,
    items: Vec<MenuItem>,
) -> Option<MenuSelection> {
    let mut selection: Option<MenuSelection> = None;
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

enum MenuItem {
    Screen(WidgetText, Screens),
    Action(WidgetText, Actions),
}

impl MenuItem {
    fn screen(s: impl Into<WidgetText>, screen: Screens) -> Self {
        MenuItem::Screen(s.into(), screen)
    }

    fn action(s: impl Into<WidgetText>, action: Actions) -> Self {
        MenuItem::Action(s.into(), action)
    }

    fn as_selection(&self) -> MenuSelection {
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

struct VerticalMenu<'a> {
    // Each menu needs a distinct id
    id: Id,
    // The last cursor direction we saw
    cursor_direction: Option<CursorDirection>,
    // The items in the menu
    items: Vec<MenuItem>,
    // selection
    selection: &'a mut Option<MenuSelection>,
}

impl<'a> Widget for VerticalMenu<'a> {
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
                .get_temp::<MenuSelection>(id)
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
                    selected = item_selection;
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
