use super::Widget;
use crate::style::StyleEntry;
use crate::types::WidgetText;
use bevy::prelude::*;

pub struct LabelWidget<'a> {
    text: &'a WidgetText,
    style: &'a StyleEntry,
}

impl<'a> LabelWidget<'a> {
    pub fn new(text: &'a WidgetText, style: &'a StyleEntry) -> Self {
        Self { text, style }
    }
}

impl<'a> Widget for LabelWidget<'a> {
    fn build(self, parent: &mut ChildBuilder) {
        let LabelWidget { text, style, .. } = self;

        let (bg, fg) = (style.normal.bg, style.selected.fg);

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
            .with_children(|parent| {
                parent.spawn(text.bundle(&text_style));
            });
    }
}
