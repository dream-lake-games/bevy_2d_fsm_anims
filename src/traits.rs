use bevy::{
    prelude::Resource,
    reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath},
};

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
{
    fn all() -> Vec<Self>;

    fn get_time_class() -> Option<i32>;

    fn get_body(&self) -> AnimBody;

    fn get_next(&self) -> AnimNextState<Self>;
}

pub trait AnimTimeProvider: Resource + Default {
    fn get_delta(&self, class: i32) -> f32;
}
