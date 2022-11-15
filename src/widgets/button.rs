use super::Widget;
use crate::style::StyleEntry;
use crate::types::{ButtonComponent, WidgetText};
use crate::{ActionTrait, MenuSelection, ScreenTrait};
use bevy::prelude::*;

pub struct ButtonWidget<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    text: &'a WidgetText,
    style: &'a StyleEntry,
    menu_identifier: (&'static str, usize),
    selection: &'a MenuSelection<A, S, State>,
    selected: bool,
}

impl<'a, A, S, State> ButtonWidget<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn new(
        text: &'a WidgetText,
        style: &'a StyleEntry,
        menu_identifier: (&'static str, usize),
        selection: &'a MenuSelection<A, S, State>,
        selected: bool,
    ) -> Self {
        Self {
            text,
            style,
            menu_identifier,
            selection,
            selected,
        }
    }
}

impl<'a, A, S, State> Widget for ButtonWidget<'a, A, S, State>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    fn build(self, parent: &mut ChildBuilder) {
        let ButtonWidget {
            text,
            style,
            menu_identifier,
            selection,
            selected,
        } = self;

        let (bg, fg) = if selected {
            (style.selected.bg, style.selected.fg)
        } else {
            (style.normal.bg, style.normal.fg)
        };

        let text_style = TextStyle {
            font: style.font.clone(),
            font_size: style.size,
            color: fg,
        };

        parent
            .spawn(ButtonBundle {
                style: Style {
                    margin: style.margin,
                    padding: style.padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(bg),
                ..default()
            })
            .insert(ButtonComponent {
                style: style.clone(),
                selection: selection.clone(),
                menu_identifier,
                selected,
            })
            .with_children(|parent| {
                parent.spawn(text.bundle(&text_style));
            });
    }
}
