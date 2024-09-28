use bevy::reflect::Reflect;
use bevy::{prelude::*, utils::HashMap};

use crate::lazy::impl_get_copy;
use crate::traits::{AnimStateMachine, ManageAnims};

/// The main animation controller
#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimMan<StateMachine: AnimStateMachine> {
    /// Current state of the animation
    pub(crate) state: StateMachine,
    /// Last state of the animation
    pub(crate) last_state: Option<StateMachine>,
    /// Flips x-axis of the animation
    pub(crate) flip_x: bool,
    /// Flips y-axis of the animation
    pub(crate) flip_y: bool,
    /// Should the `AnimStateChange` event be triggered?
    pub(crate) observe_state_changes: bool,
    /// Should the `AnimIxChange` event be triggered?
    pub(crate) observe_ix_changes: bool,
    /// INTERNAL: The entities of the spawned body children
    pub(crate) tagged_children: HashMap<StateMachine, Entity>,
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            last_state: None,
            flip_x: false,
            flip_y: false,
            observe_state_changes: false,
            observe_ix_changes: false,
            tagged_children: default(),
        }
    }
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    impl_get_copy!(state, StateMachine);
    impl_get_copy!(flip_x, bool);
    impl_get_copy!(flip_y, bool);
}
// This mutability hack exists so that `Changed` has good meaning.
// In bevy, dereferencing a mutable pointer trigggers change. So we want to have a way to make
// set and what not not actually trigger Change unless the values, well, change.
macro_rules! impl_mutable_animation_manager_field {
    ($field:ident, $type:ty) => {
        paste::paste! {
            fn [<set_ $field>](&mut self, val: $type) {
                if val == self.$field {
                    return;
                }
                self.$field = val;
            }
            fn [<reset_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        }
    };
}
impl<'w, StateMachine: AnimStateMachine> ManageAnims<StateMachine>
    for Mut<'w, AnimMan<StateMachine>>
{
    fn set_state(&mut self, val: StateMachine) {
        if val == self.state {
            return;
        }
        self.last_state = Some(self.state);
        self.state = val;
    }
    fn reset_state(&mut self, val: StateMachine) {
        self.last_state = Some(self.state);
        self.state = val;
    }
    impl_mutable_animation_manager_field!(flip_x, bool);
    impl_mutable_animation_manager_field!(flip_y, bool);
}
