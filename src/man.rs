use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

use crate::lazy::{impl_get_copy, impl_with};
use crate::traits::{AnimStateMachine, ManageAnims};

#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimMan<StateMachine: AnimStateMachine> {
    pub state: StateMachine,
    pub hidden: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub play_while_paused: bool,
    // Need to keep handles to loaded assets around so switching doesn't blink
    pub handle_cache: HashMap<StateMachine::BodyType, Handle<Image>>,
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            hidden: false,
            flip_x: false,
            flip_y: false,
            play_while_paused: false,
            handle_cache: default(),
        }
    }

    impl_get_copy!(state, StateMachine);
    impl_with!(state, StateMachine);
    impl_get_copy!(hidden, bool);
    impl_with!(hidden, bool);
    impl_get_copy!(flip_x, bool);
    impl_with!(flip_x, bool);
    impl_get_copy!(flip_y, bool);
    impl_with!(flip_y, bool);
    impl_get_copy!(play_while_paused, bool);
    impl_with!(play_while_paused, bool);
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
    impl_mutable_animation_manager_field!(state, StateMachine);
    impl_mutable_animation_manager_field!(hidden, bool);
    impl_mutable_animation_manager_field!(flip_x, bool);
    impl_mutable_animation_manager_field!(flip_y, bool);
}
