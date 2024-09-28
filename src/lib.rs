use bevy::{prelude::*, render::view::RenderLayers};
use man::AnimMan;
use state::AnimState;

pub mod prelude {
    pub use crate::man::AnimMan;
    pub use crate::plugin::*;
    pub use crate::traits::*;
    pub use crate::{AnimBody, AnimBundle, AnimIxChange, AnimNextState, AnimSet, AnimStateChange};
}

mod body;
mod lazy;
mod logic;
mod man;
mod mat;
mod plugin;
mod state;
mod traits;

use crate::traits::AnimStateMachine;

/// The main animation bundle
#[derive(Bundle)]
pub struct AnimBundle<StateMachine: AnimStateMachine> {
    man: AnimMan<StateMachine>,
    state: AnimState<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Default for AnimBundle<StateMachine> {
    fn default() -> Self {
        Self {
            man: AnimMan::new(),
            state: AnimState::new(),
        }
    }
}
impl<StateMachine: AnimStateMachine> AnimBundle<StateMachine> {
    pub fn new(state: StateMachine) -> Self {
        Self::default().with_state(state)
    }
    pub fn with_state(mut self, state: StateMachine) -> Self {
        self.man.state = state;
        self.state.state = state;
        self
    }
    pub fn with_flip_x(mut self, flip_x: bool) -> Self {
        self.man.flip_x = flip_x;
        self.state.flip_x = flip_x;
        self
    }
    pub fn with_flip_y(mut self, flip_y: bool) -> Self {
        self.man.flip_y = flip_y;
        self.state.flip_y = flip_y;
        self
    }
    pub fn with_observe_state(mut self) -> Self {
        self.state.observe_state_changes = true;
        self
    }
    pub fn with_observe_ix(mut self) -> Self {
        self.state.observe_ix_changes = true;
        self
    }
}

/// A schedule set containing all logic for updating and playing animations
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimSet;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum AnimNextState<NextType> {
    Stay,
    Some(NextType),
    Despawn,
}

/// An event that is triggered when the given state changes. Must be observed.
#[derive(Event, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct AnimStateChange<StateMachine: AnimStateMachine> {
    pub prev: Option<StateMachine>,
    pub state: StateMachine,
}

/// An event that is triggered when the ix of the current animation changes. Must be observed.
#[derive(Event, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct AnimIxChange<StateMachine: AnimStateMachine> {
    pub state: StateMachine,
    pub ix: u32,
}
