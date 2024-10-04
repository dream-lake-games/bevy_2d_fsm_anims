use std::marker::PhantomData;

use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;

use crate::lazy::impl_with;
use crate::logic::register_logic;
use crate::man::AnimMan;
use crate::time::AnimPlaceholderTime;
use crate::traits::{AnimStateMachine, AnimTimeProvider};

pub const DEFAULT_TIME_CLASS: i32 = 0;

#[derive(Default)]
pub struct AnimDefnPlugin<
    StateMachine: AnimStateMachine,
    AnimTime: AnimTimeProvider = AnimPlaceholderTime,
> {
    _pd: PhantomData<(StateMachine, AnimTime)>,
}
impl<StateMachine: AnimStateMachine, AnimTime: AnimTimeProvider> Plugin
    for AnimDefnPlugin<StateMachine, AnimTime>
{
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine, AnimTime>(app);
        app.register_type::<AnimMan<StateMachine>>();
    }
}

#[derive(Clone, Debug, Reflect, Resource)]
pub(crate) struct AnimDefaults {
    pub default_fps: f32,
    pub default_render_layers: RenderLayers,
    pub default_time_class: i32,
}

pub(crate) fn update_placeholder_time(
    time: Res<Time>,
    mut placeholder_time: ResMut<AnimPlaceholderTime>,
) {
    placeholder_time.real_time_delta = time.delta_seconds();
}

pub struct AnimPlugin<AnimTime: AnimTimeProvider = AnimPlaceholderTime> {
    default_fps: f32,
    default_render_layers: RenderLayers,
    _pd: PhantomData<AnimTime>,
}
impl AnimPlugin<AnimPlaceholderTime> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<AnimTime: AnimTimeProvider> AnimPlugin<AnimTime> {
    impl_with!(default_fps, f32);
    impl_with!(default_render_layers, RenderLayers);
}
impl<AnimTime: AnimTimeProvider> Default for AnimPlugin<AnimTime> {
    fn default() -> Self {
        Self {
            default_fps: 24.0,
            default_render_layers: RenderLayers::default(),
            _pd: default(),
        }
    }
}
impl<AnimTime: AnimTimeProvider> Plugin for AnimPlugin<AnimTime> {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "anim_mat.wgsl");
        app.add_plugins(Material2dPlugin::<crate::mat::AnimMat>::default());
        app.insert_resource(AnimDefaults {
            default_fps: self.default_fps,
            default_render_layers: self.default_render_layers.clone(),
            default_time_class: DEFAULT_TIME_CLASS,
        });
        app.insert_resource(AnimPlaceholderTime::default());

        app.add_systems(Update, update_placeholder_time.in_set(crate::AnimSet));
    }
}
