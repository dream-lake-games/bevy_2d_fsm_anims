use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};

use crate::lazy::{impl_get_ref, impl_with, impl_with_option};
use crate::mat::AnimMat;
use crate::plugin::AnimDefaults;
use crate::traits::AnimStateMachine;
use crate::{AnimIndex, AnimNextState};

#[derive(Debug, Clone, Reflect)]
pub struct AnimBodyData {
    path: String,
    size: UVec2,
    length: u32,
    fps: Option<f32>,
    offset: Vec2,
    zix: f32,
    render_layers: Option<RenderLayers>,
}
impl AnimBodyData {
    pub fn new(path: &str, width: u32, height: u32) -> Self {
        Self {
            path: path.into(),
            size: UVec2::new(width, height),
            length: 1,
            fps: None,
            offset: Vec2::default(),
            zix: 0.0,
            render_layers: None,
        }
    }

    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = Vec2::new(x, y);
        self
    }

    impl_with!(length, u32);
    impl_with_option!(fps, f32);
    impl_with!(zix, f32);
    impl_with_option!(render_layers, RenderLayers);

    impl_get_ref!(path, String);
}

#[derive(Bundle, Clone)]
pub(crate) struct AnimBodyDataBundle<StateMachine: AnimStateMachine> {
    name: Name,
    mesh: Mesh2dHandle,
    material: Handle<AnimMat>,
    spatial: SpatialBundle,
    render_layers: RenderLayers,
    index: AnimIndex<StateMachine>,
}
impl<StateMachine: AnimStateMachine> AnimBodyDataBundle<StateMachine> {
    pub(crate) fn new(
        body_type: StateMachine::BodyType,
        data: AnimBodyData,
        next: AnimNextState<StateMachine>,
        flip_x: bool,
        flip_y: bool,
        ass: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        mats: &mut ResMut<Assets<AnimMat>>,
        defaults: &Res<AnimDefaults>,
    ) -> Self {
        let mesh = Mesh::from(Rectangle::new(data.size.x as f32, data.size.y as f32));
        Self {
            name: Name::new(format!("AnimBodyDataBundle_{body_type:?}")),
            mesh: meshes.add(mesh).into(),
            material: mats.add(AnimMat::new(
                // ass.load(data.path),
                ass.load(data.path),
                data.length,
                flip_x,
                flip_y,
            )),
            spatial: SpatialBundle::from_transform(Transform {
                translation: data.offset.extend(data.zix),
                ..default()
            }),
            render_layers: data
                .render_layers
                .unwrap_or(defaults.default_render_layers.clone()),
            index: AnimIndex {
                body_type,
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0 / data.fps.unwrap_or(defaults.default_fps),
                next,
            },
        }
    }
}
