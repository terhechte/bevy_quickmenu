use crate::{
    style::{StyleEntry, Stylesheet},
    types::{ButtonComponent, MenuIcon, MenuItem, MenuSelection, NavigationEvent, Selections},
    ActionTrait, ScreenTrait,
};
use bevy::prelude::*;
// use bevy_egui::egui::{Id, Response, Sense, Ui, Vec2, Widget};

//use super::{BorderedButton, BorderedLabel};
use super::BorderedLabel;
use super::Widget;

pub struct VerticalMenu<'a, State, A, S>
where
    A: ActionTrait<State = State>,
    S: ScreenTrait<Action = A>,
{
    // // Each menu needs a distinct id
    pub id: &'static str,
    // The last cursor direction we saw
    // pub cursor_direction: Option<NavigationEvent>,
    // The items in the menu
    pub items: &'a [MenuItem<State, A, S>],
    // selection
    // pub selection: &'a mut Option<MenuSelection<A, S, State>>,
    // stylesheet
    pub stylesheet: &'a Stylesheet,
}

impl<'a, State, A, S> VerticalMenu<'a, State, A, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    pub fn build(self, selections: &Selections, builder: &mut ChildBuilder, active: bool) {
        let VerticalMenu {
            id,
            items,
            stylesheet,
            ..
        } = self;
        if items.is_empty() {
            return;
            // return ui.allocate_response(Vec2::ZERO, Sense::click());
        }

        builder
            .spawn(NodeBundle {
                style: Style {
                    // size: Size::new(Val::Percent(100.0), Val::Px(30.)),
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(stylesheet.vertical_spacing)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let (mut selected_idx, selectables) =
                    Self::current_selection(id, items, selections);
                // get the selectable items
                // let selectables: Vec<_> = items
                //     .iter()
                //     .enumerate()
                //     .filter(|(_, e)| e.is_selectable())
                //     .collect();

                // let mut selected_idx = selections.0.get(id).copied().unwrap_or_else(|| {
                //     // First, try to find the first non-None selection,
                //     // otherwise, take the first
                //     let non_none = selectables
                //         .iter()
                //         .find(|(_, e)| e.as_selection() != MenuSelection::None)
                //         .map(|(i, _)| *i);
                //     non_none.unwrap_or_else(|| 0)
                // });
                // println!("Selected idx {selected_idx}");

                let mut selected = selectables[selected_idx].1.as_selection();

                // let mut select_navigation = false;

                // if let Some(cursor_direction) = cursor_direction {
                //     println!("GOT CURSOR {cursor_direction:?}");
                //     let mut index = selected_idx;
                //     match cursor_direction {
                //         NavigationEvent::Up if index > 0 => index -= 1,
                //         NavigationEvent::Down if index < (selectables.len() - 1) => index += 1,
                //         // NavigationEvent::Select => select_navigation = true,
                //         _ => (),
                //     }
                //     if selectables[index].1.as_selection() != MenuSelection::None {
                //         println!("Apply new selection");
                //         selected_idx = index;
                //         selected = selectables[selected_idx].1.as_selection();
                //     }
                //     // change selection
                //     // println!("change {selected_idx} to {index}");
                //     // selected_idx = index;
                // }

                let mut index = 0;
                for item in items {
                    let is_label = matches!(item, MenuItem::Label(_, _));
                    let item_selection = item.as_selection();
                    let focussed = (selected == item_selection) && !is_label;

                    let response = match item {
                        MenuItem::Screen(t, i, _) => self.add_item(
                            parent,
                            i,
                            &stylesheet.button,
                            &item_selection,
                            (id, index),
                            focussed,
                            active,
                            //BorderedButton::new(t, &stylesheet.button).set_focus(focussed),
                            BorderedLabel::new(t, &stylesheet.label, &item_selection),
                        ),
                        MenuItem::Action(t, i, _) => self.add_item(
                            parent,
                            i,
                            &stylesheet.button,
                            &item_selection,
                            (id, index),
                            focussed,
                            active,
                            //BorderedButton::new(t, &stylesheet.button).set_focus(focussed),
                            BorderedLabel::new(t, &stylesheet.label, &item_selection),
                        ),
                        MenuItem::Label(t, i) => self.add_item(
                            parent,
                            i,
                            &stylesheet.label,
                            &item_selection,
                            (id, index),
                            focussed,
                            active,
                            BorderedLabel::new(t, &stylesheet.label, &item_selection),
                        ),
                        MenuItem::Headline(t, i) => self.add_item(
                            parent,
                            i,
                            &stylesheet.headline,
                            &item_selection,
                            (id, index),
                            focussed,
                            active,
                            BorderedLabel::new(t, &stylesheet.headline, &item_selection),
                        ),
                    };
                    // FIXME: Detect clicks!???
                    // PROBABLY BY ALSO INJECTING INTO THE EVENT PIPE WHERE KEYBOARD
                    // AND GAMEPAD EVENTS GO!
                    // if !is_label && (response.clicked() || (select_navigation && focussed)) {
                    //     selected = item_selection.clone();
                    //     *selection = Some(item_selection);
                    // }

                    // if !is_label && (select_navigation && focussed) {
                    //     println!("New selection for {id} {index}");
                    //     *self.selection = Some(item_selection);
                    //     // selected_idx = index;
                    // }

                    // FIXME: Need space here no?
                    // ui.add_space(stylesheet.vertical_spacing);
                    index += 1;
                }
                // println!("New Selected idx {selected_idx}");

                // selections.0.insert(id, selected_idx);

                // ui.memory().data.insert_temp(id, selected);

                // ui.allocate_response(Vec2::ZERO, Sense::click())
            });

        // ui.vertical(|ui| {}).inner
    }

    pub fn apply_event(
        event: &NavigationEvent,
        id: &'static str,
        items: &'a [MenuItem<State, A, S>],
        selections: &mut Selections,
    ) -> Option<MenuSelection<A, S, State>> {
        let (mut selectable_index, selectables) = Self::current_selection(id, items, selections);
        // let selectables: Vec<_> = items
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, e)| e.is_selectable())
        //     .collect();

        let mut select_navigation = false;

        // let selected_idx = selections.0.get(id).copied().unwrap_or_else(|| {
        //     let non_none = selectables
        //         .iter()
        //         .find(|(_, e)| e.as_selection() != MenuSelection::None)
        //         .map(|(i, _)| *i);
        //     non_none.unwrap_or_else(|| 0)
        // });

        //let mut index = selected_idx;
        // let mut selectable_index = selectables
        //     .iter()
        //     .find(|(i, _)| *i == selected_idx)
        //     .map(|(i, _)| *i)
        //     .unwrap_or_default();

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
            println!("Apply new selection");
            // selected_idx = selectable_index;
            selected = selectables[selectable_index].1.as_selection();
        }
        // change selection
        // println!("change {selected_idx} to {index}");
        // selected_idx = index;

        let mut index = 0;
        for item in items {
            let is_label = matches!(item, MenuItem::Label(_, _));
            let item_selection = item.as_selection();
            let focussed = (selected == item_selection) && !is_label;
            if !is_label && (select_navigation && focussed) {
                println!("New selection for {id} {index}");
                return Some(item_selection);
                // selected_idx = index;
            }

            // FIXME: Need space here no?
            // ui.add_space(stylesheet.vertical_spacing);
            index += 1;
        }
        println!("New Selected idx {selectable_index}");

        selections.0.insert(id, selectable_index);
        None
    }

    fn current_selection(
        id: &'static str,
        items: &'a [MenuItem<State, A, S>],
        selections: &Selections,
    ) -> (usize, Vec<(usize, &'a MenuItem<State, A, S>)>) {
        let selectables: Vec<_> = items
            .iter()
            //.filter(|(_, e)| e.is_selectable())
            .filter(|e| e.is_selectable())
            .enumerate()
            .collect();

        let selected_idx = selections.0.get(id).copied().unwrap_or_else(|| {
            let non_none = selectables
                .iter()
                .find(|(_, e)| e.as_selection() != MenuSelection::None)
                .map(|(i, _)| *i);
            non_none.unwrap_or_else(|| 0)
        });

        (selected_idx, selectables)
    }

    fn add_item(
        &self,
        parent: &mut ChildBuilder,
        icon: &MenuIcon,
        style: &StyleEntry,
        selection: &MenuSelection<A, S, State>,
        menu_identifier: (&'static str, usize),
        focussed: bool,
        active: bool,
        widget: impl Widget,
    ) {
        let icon_style: StyleEntry = style.as_iconstyle();
        let is_postfix = icon.is_postfix();

        // FIXME: Place the thing

        parent
            .spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    // padding: UiRect::all(Val::Px(stylesheet.vertical_spacing)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // FIXME: Add support for icon
                widget.build(parent, menu_identifier, focussed, active);
                // FIXME: Add suppot for icon
            });

        // ui.horizontal(|ui| {
        //     if !is_postfix {
        //         if let Some(t) = icon.icon() {
        //             ui.add(BorderedLabel::new(&t.into(), &icon_style));
        //             ui.add_space(style.icon_style.leading_margin);
        //         }
        //     }
        //     let response = ui.add(widget);
        //     if is_postfix {
        //         if let Some(t) = icon.icon() {
        //             ui.add_space(style.icon_style.trailing_margin);
        //             ui.add(BorderedLabel::new(&t.into(), &icon_style));
        //         }
        //     }
        //     response
        // })
        // .inner
    }
}
