use crate::{
    style::{StyleEntry, Stylesheet},
    types::{
        MenuAssets, MenuIcon, MenuItem, MenuSelection, NavigationEvent, Selections,
        VerticalMenuComponent, WidgetId,
    },
    ScreenTrait,
};
use bevy::prelude::*;

use super::Widget;
use super::{ButtonWidget, LabelWidget};

pub struct VerticalMenu<'a, S>
where
    S: ScreenTrait,
{
    // // Each menu needs a distinct id
    pub id: WidgetId,
    // The items in the menu
    pub items: &'a [MenuItem<S>],
    // Our Stylesheet
    pub stylesheet: &'a Stylesheet,
    // Assets
    pub assets: &'a MenuAssets,
    // Overriding Bevy Style
    pub style: Option<&'a Style>,
    // Overriding Bevy Background Color
    pub background: Option<&'a BackgroundColor>,
}

impl<'a, S> VerticalMenu<'a, S>
where
    S: ScreenTrait + 'static,
{
    pub fn build(self, selections: &Selections, builder: &mut ChildBuilder) {
        let VerticalMenu {
            id,
            items,
            stylesheet,
            assets,
            ..
        } = self;
        if items.is_empty() {
            return;
        }

        let style = self.style.cloned().unwrap_or_else(|| Style {
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(stylesheet.vertical_spacing)),
            ..default()
        });

        let background_color = self
            .background
            .cloned()
            .unwrap_or_else(|| Color::NONE.into());

        builder
            .spawn(NodeBundle {
                style,
                background_color,
                ..default()
            })
            .with_children(|parent| {
                let (selected_idx, selectables) = Self::current_selection(&id, items, selections);

                let selected = selectables[selected_idx].1.as_selection();

                let mut index = 0;
                for item in items {
                    let is_label = matches!(item, MenuItem::Label(_, _));
                    let item_selection = item.as_selection();
                    let focussed = (selected == item_selection) && !is_label;

                    match item {
                        MenuItem::Screen(t, i, _) => Self::add_item(
                            assets,
                            parent,
                            i,
                            &stylesheet.button,
                            ButtonWidget::new(
                                t,
                                &stylesheet.button,
                                (id.clone(), index),
                                &item_selection,
                                focussed,
                            ),
                        ),
                        MenuItem::Action(t, i, _) => Self::add_item(
                            assets,
                            parent,
                            i,
                            &stylesheet.button,
                            ButtonWidget::new(
                                t,
                                &stylesheet.button,
                                (id.clone(), index),
                                &item_selection,
                                focussed,
                            ),
                        ),
                        MenuItem::Label(t, i) => Self::add_item(
                            assets,
                            parent,
                            i,
                            &stylesheet.label,
                            LabelWidget::new(t, &stylesheet.label),
                        ),
                        MenuItem::Headline(t, i) => Self::add_item(
                            assets,
                            parent,
                            i,
                            &stylesheet.headline,
                            LabelWidget::new(t, &stylesheet.headline),
                        ),
                        MenuItem::Image(i, s) => {
                            let style = s.clone().unwrap_or_else(|| Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            });
                            parent.spawn(ImageBundle {
                                style,
                                image: i.clone().into(),
                                ..Default::default()
                            });
                        }
                    };

                    // Only increase for menu elements, so the indexes pair up
                    // with the `selectables` indexes
                    if item_selection != MenuSelection::None {
                        index += 1;
                    }
                }
            })
            .insert(VerticalMenuComponent(id));
    }

    pub fn apply_event(
        event: &NavigationEvent,
        id: WidgetId,
        items: &'a [MenuItem<S>],
        selections: &mut Selections,
    ) -> Option<MenuSelection<S>> {
        let (mut selectable_index, selectables) = Self::current_selection(&id, items, selections);

        let mut select_navigation = false;

        let mut selected = selectables[selectable_index].1.as_selection();

        match event {
            NavigationEvent::Up if selectable_index > 0 => selectable_index -= 1,
            NavigationEvent::Down if selectable_index < (selectables.len() - 1) => {
                selectable_index += 1
            }
            NavigationEvent::Select => select_navigation = true,
            _ => (),
        }

        if selectables[selectable_index].1.as_selection() != MenuSelection::None {
            selected = selectables[selectable_index].1.as_selection();
        }
        for item in items {
            let is_label = matches!(item, MenuItem::Label(_, _));
            let item_selection = item.as_selection();
            let focussed = (selected == item_selection) && !is_label;
            if !is_label && (select_navigation && focussed) {
                return Some(item_selection);
            }
        }
        selections.0.insert(id, selectable_index);
        None
    }

    #[allow(clippy::type_complexity)]
    fn current_selection(
        id: &WidgetId,
        items: &'a [MenuItem<S>],
        selections: &Selections,
    ) -> (usize, Vec<(usize, &'a MenuItem<S>)>) {
        let selectables: Vec<_> = items
            .iter()
            .filter(|e| e.is_selectable())
            .enumerate()
            .collect();

        let selected_idx = selections.0.get(id).copied().unwrap_or_else(|| {
            let non_none = selectables
                .iter()
                .find(|(_, e)| e.as_selection() != MenuSelection::None)
                .map(|(i, _)| *i);
            non_none.unwrap_or_default()
        });

        (selected_idx, selectables)
    }

    fn add_item(
        assets: &MenuAssets,
        parent: &mut ChildBuilder,
        icon: &MenuIcon,
        style: &StyleEntry,
        widget: impl Widget,
    ) {
        parent
            .spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                if let Some(image_handle) = icon.resolve_icon(assets) {
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: style.icon_style.width,
                            height: style.icon_style.height,
                            margin: style.icon_style.padding,
                            ..default()
                        },
                        image: image_handle.into(),
                        background_color: BackgroundColor(style.icon_style.tint_color),
                        ..Default::default()
                    });
                }
                widget.build(parent, assets);
            });
    }
}
