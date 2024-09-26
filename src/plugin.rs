use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

use crate::logic::register_logic;
use crate::traits::AnimStateMachine;

pub struct AnimDefnPlugin<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Plugin for AnimDefnPlugin<StateMachine> {
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine>(app);
    }
}

pub struct AnimPlugin;
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<crate::mat::AnimMat>::default());
    }
}
