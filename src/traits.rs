use bevy::reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath};

use crate::{body::AnimBodyData, AnimNextState};

pub trait AnimBody:
    Sized
    + Send
    + Sync
    + 'static
    + std::hash::Hash
    + PartialEq
    + Eq
    + Copy
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
    + std::fmt::Debug
{
    fn all_bodies() -> Vec<Self>;

    fn to_body_data(&self) -> AnimBodyData;
}

pub trait ManageAnims<StateMachine: AnimStateMachine> {
    /// Sets the currently value of the animation manager state, doing nothing if the value is the same
    fn set_state(&mut self, state: StateMachine);
    /// Resets the currently value of the animation manager state, triggering change even if the value is the same
    fn reset_state(&mut self, state: StateMachine);
    /// Sets the currently value of the animation manager hidden, doing nothing if the value is the same
    fn set_hidden(&mut self, hidden: bool);
    /// Resets the currently value of the animation manager hidden, triggering change even if the value is the same
    fn reset_hidden(&mut self, hidden: bool);
    /// Sets the currently value of the animation manager flip_x, doing nothing if the value is the same
    fn set_flip_x(&mut self, flip_x: bool);
    /// Resets the currently value of the animation manager flip_x, triggering change even if the value is the same
    fn reset_flip_x(&mut self, flip_x: bool);
    /// Sets the currently value of the animation manager flip_y, doing nothing if the value is the same
    fn set_flip_y(&mut self, flip_y: bool);
    /// Resets the currently value of the animation manager flip_y, triggering change even if the value is the same
    fn reset_flip_y(&mut self, flip_y: bool);
}

pub trait AnimStateMachine:
    Sized
    + Send
    + Sync
    + 'static
    + std::hash::Hash
    + PartialEq
    + Eq
    + Copy
    + Default
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
    + std::fmt::Debug
{
    type BodyType: AnimBody;

    fn all_states() -> Vec<Self>;

    fn next(&self) -> AnimNextState<Self>;
}
