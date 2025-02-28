//! [`Pane`] module.

use std::marker::PhantomData;

use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

use crate::prelude::PaneStructure;

/// Trait for pane definitions
pub trait Pane: Component {
    /// The parameter(s) used in the creation function.
    type Param: SystemParam + 'static;

    /// The default name displayed in the pane's header
    const NAME: &str;

    /// The id that the pane will be serialized under in saved formats
    const ID: &str;

    /// Similar to Plugin::build, this code is run when the Pane is registered
    fn build(app: &mut App);

    /// A system that should be run when the Pane is added. Should create the pane's content.
    fn on_create(pane: In<PaneStructure>, param: StaticSystemParam<Self::Param>);
}

#[derive(Default)]
pub(crate) struct PanePlugin<T: Pane> {
    marker: PhantomData<T>,
}

impl<T: Pane> PanePlugin<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T: Pane> Plugin for PanePlugin<T> {
    fn build(&self, app: &mut App) {
        T::build(app)
    }

    fn is_unique(&self) -> bool {
        false
    }
}
