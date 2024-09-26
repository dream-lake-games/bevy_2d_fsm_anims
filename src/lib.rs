use bevy::prelude::*;
use bevy::utils::HashMap;

pub mod prelude {
    pub use crate::body::AnimBodyData;
    pub use crate::man::AnimMan;
    pub use crate::plugin::*;
    pub use crate::traits::*;
    pub use crate::{AnimBodyProgress, AnimNextState, AnimSet};
}

mod body;
mod lazy;
mod logic;
mod man;
mod mat;
mod plugin;
mod traits;

use crate::traits::AnimStateMachine;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimSet;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum AnimNextState<NextType> {
    None,
    Some(NextType),
    Despawn,
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimBodyProgress<StateMachine: AnimStateMachine> {
    pub ixes: HashMap<StateMachine::BodyType, u32>,
}
impl<StateMachine: AnimStateMachine> AnimBodyProgress<StateMachine> {
    pub fn get_body_ix(&self, body_type: StateMachine::BodyType) -> Option<u32> {
        self.ixes.get(&body_type).map(|thing| *thing)
    }
}

/// For tracking animations that play
#[derive(Component, Debug, Clone, Reflect)]
struct AnimIndex<StateMachine: AnimStateMachine> {
    body_type: StateMachine::BodyType,
    ix: u32,
    length: u32,
    time: f32,
    /// Seconds per frame
    spf: f32,
    /// The state to transition to after this state. Note that this has a None variant inside it.
    next: AnimNextState<StateMachine>,
}

/// Attached to the body of the animation that (when finished) triggers the state transition
#[derive(Component, Debug, Clone, Reflect)]
struct AnimNextBurden<StateMachine: AnimStateMachine> {
    next_state: AnimNextState<StateMachine>,
}
