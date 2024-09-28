use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;

use crate::logic::register_logic;
use crate::man::AnimMan;
use crate::state::AnimState;
use crate::traits::AnimStateMachine;
use crate::AnimIndex;

#[derive(Default)]
pub struct AnimDefnPlugin<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}
impl<StateMachine: AnimStateMachine> Plugin for AnimDefnPlugin<StateMachine> {
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine>(app);
        app.register_type::<AnimMan<StateMachine>>();
        app.register_type::<AnimState<StateMachine>>();
        app.register_type::<AnimIndex<StateMachine>>();
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub(crate) struct AnimDefaults {
    pub default_fps: f32,
    pub default_render_layers: RenderLayers,
}

pub struct AnimPlugin {
    pub default_fps: f32,
    pub default_render_layers: RenderLayers,
}
impl Default for AnimPlugin {
    fn default() -> Self {
        Self {
            default_fps: 24.0,
            default_render_layers: RenderLayers::default(),
        }
    }
}
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimDefaults {
            default_fps: self.default_fps,
            default_render_layers: self.default_render_layers.clone(),
        });
        app.add_plugins(Material2dPlugin::<crate::mat::AnimMat>::default());
    }
}
