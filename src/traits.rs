use bevy::reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath};
use strum::IntoEnumIterator;

use crate::{body::AnimBody, AnimNextState};

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
    + IntoEnumIterator
{
    fn get_data(&self) -> AnimBody;

    fn get_next(&self) -> AnimNextState<Self>;
}
