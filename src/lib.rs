use bevy::prelude::*;

pub mod prelude {
    pub use crate::body::AnimBody;
    pub use crate::man::AnimMan;
    pub use crate::plugin::*;
    pub use crate::traits::*;
    pub use crate::{AnimIxChange, AnimNextState, AnimSet, AnimStateChange};
}

mod body;
mod lazy;
mod logic;
mod man;
mod mat;
mod plugin;
mod traits;

use crate::traits::AnimStateMachine;

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
    pub next: StateMachine,
}

/// An event that is triggered when the ix of the current animation changes. Must be observed.
#[derive(Event, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct AnimIxChange<StateMachine: AnimStateMachine> {
    pub state: StateMachine,
    pub ix: u32,
}
