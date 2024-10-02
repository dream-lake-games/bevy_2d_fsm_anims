use std::marker::PhantomData;

use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;

use crate::lazy::impl_with;
use crate::logic::register_logic;
use crate::man::AnimMan;
use crate::time::{AnimPlaceholderTimeClass, AnimPlaceholderTimeRes};
use crate::traits::{AnimStateMachine, AnimTimeClassTrait, AnimTimeResTrait};

#[derive(Default)]
pub struct AnimDefnPlugin<
    StateMachine: AnimStateMachine,
    AnimTimeClass: AnimTimeClassTrait = AnimPlaceholderTimeClass,
    AnimTimeRes: AnimTimeResTrait<AnimTimeClass> = AnimPlaceholderTimeRes,
> {
    _pd: PhantomData<(StateMachine, AnimTimeClass, AnimTimeRes)>,
}
impl<
        StateMachine: AnimStateMachine,
        AnimTimeClass: AnimTimeClassTrait,
        AnimTimeRes: AnimTimeResTrait<AnimTimeClass>,
    > Plugin for AnimDefnPlugin<StateMachine, AnimTimeClass, AnimTimeRes>
{
    fn build(&self, app: &mut App) {
        register_logic::<StateMachine, AnimTimeClass, AnimTimeRes>(app);
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
    mut placeholder_time: ResMut<AnimPlaceholderTimeRes>,
) {
    placeholder_time.real_time_delta = time.delta_seconds();
}

pub struct AnimPlugin<
    TimeClass: AnimTimeClassTrait = AnimPlaceholderTimeClass,
    TimeRes: AnimTimeResTrait<TimeClass> = AnimPlaceholderTimeRes,
> {
    default_fps: f32,
    default_render_layers: RenderLayers,
    _pd: PhantomData<(TimeClass, TimeRes)>,
}
impl AnimPlugin<AnimPlaceholderTimeClass, AnimPlaceholderTimeRes> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<TimeClass: AnimTimeClassTrait, TimeRes: AnimTimeResTrait<TimeClass>>
    AnimPlugin<TimeClass, TimeRes>
{
    impl_with!(default_fps, f32);
    impl_with!(default_render_layers, RenderLayers);
}
impl<TimeClass: AnimTimeClassTrait, TimeRes: AnimTimeResTrait<TimeClass>> Default
    for AnimPlugin<TimeClass, TimeRes>
{
    fn default() -> Self {
        Self {
            default_fps: 24.0,
            default_render_layers: RenderLayers::default(),
            _pd: default(),
        }
    }
}
impl<AnimTimeClass: AnimTimeClassTrait, AnimTimeRes: AnimTimeResTrait<AnimTimeClass>> Plugin
    for AnimPlugin<AnimTimeClass, AnimTimeRes>
{
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "anim_mat.wgsl");
        app.add_plugins(Material2dPlugin::<crate::mat::AnimMat>::default());
        app.insert_resource(AnimDefaults {
            default_fps: self.default_fps,
            default_render_layers: self.default_render_layers.clone(),
            default_time_class: AnimTimeClass::default().into(),
        });
        app.insert_resource(AnimPlaceholderTimeRes::default());

        app.add_systems(Update, update_placeholder_time.in_set(crate::AnimSet));
    }
}
