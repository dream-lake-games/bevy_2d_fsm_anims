use bevy::prelude::Resource;

use crate::{plugin::DEFAULT_TIME_CLASS, traits::AnimTimeProvider};

#[derive(Resource, Default)]
pub struct AnimPlaceholderTime {
    pub(crate) real_time_delta: f32,
}
impl AnimTimeProvider for AnimPlaceholderTime {
    fn get_delta(&self, class: i32) -> f32 {
        match class {
            DEFAULT_TIME_CLASS => self.real_time_delta,
            wrong => panic!("Unsupported time class: {wrong}. The placeholder time only supportes one time class, {DEFAULT_TIME_CLASS}. For more, add your own AnimTimeProvider Resource"),
        }
    }
}
