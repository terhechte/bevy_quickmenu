use bevy::prelude::ChildBuilder;

use crate::{ActionTrait, ScreenTrait};

pub trait Widget {
    type State: 'static;
    type A: ActionTrait<State = Self::State> + 'static;
    type S: ScreenTrait<Action = Self::A> + 'static;

    fn build(
        self,
        parent: &mut ChildBuilder,
        menu_identifier: (&'static str, usize),
        selected: bool,
        active: bool,
    );
}

// pub trait InteractiveWidget {
//     type State: 'static;
//     type A: ActionTrait<State = Self::State> + 'static;
//     type S: ScreenTrait<Action = Self::A> + 'static;

//     fn build(self, parent: &mut ChildBuilder, );
// }
