use bevy::prelude::ChildBuilder;

pub trait Widget {
    fn build(self, parent: &mut ChildBuilder);
}
