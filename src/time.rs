use bevy::prelude::Resource;

use crate::traits::{AnimTimeClassTrait, AnimTimeResTrait};

#[derive(Default)]
pub enum AnimPlaceholderTimeClass {
    #[default]
    Realtime = 1,
}
impl From<i32> for AnimPlaceholderTimeClass {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::Realtime,
            _ => unreachable!(),
        }
    }
}
impl Into<i32> for AnimPlaceholderTimeClass {
    fn into(self) -> i32 {
        match self {
            Self::Realtime => 1,
        }
    }
}
impl AnimTimeClassTrait for AnimPlaceholderTimeClass {}

#[derive(Resource, Default)]
pub struct AnimPlaceholderTimeRes {
    pub(crate) real_time_delta: f32,
}
impl AnimTimeResTrait<AnimPlaceholderTimeClass> for AnimPlaceholderTimeRes {
    fn get_delta(&self, _class: AnimPlaceholderTimeClass) -> f32 {
        self.real_time_delta
    }
}
