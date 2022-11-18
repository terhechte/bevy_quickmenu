use bevy::prelude::ChildBuilder;

use crate::types::MenuAssets;

pub trait Widget {
    fn build(self, parent: &mut ChildBuilder, assets: &MenuAssets);
}
