use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};

use crate::mat::AnimMat;
use crate::traits::AnimStateMachine;
use crate::{AnimIndex, AnimNextState};

#[derive(Debug, Clone, Reflect)]
pub struct AnimBodyData {
    pub path: String,
    pub size: UVec2,
    pub length: u32,
    pub fps: f32,
    pub offset: Vec2,
    pub zix: f32,
    pub render_layers: RenderLayers,
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
        ass: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        mats: &mut ResMut<Assets<AnimMat>>,
        flip_x: bool,
        flip_y: bool,
    ) -> Self {
        let mesh = Mesh::from(Rectangle::new(data.size.x as f32, data.size.y as f32));
        Self {
            name: Name::new("body_data_bundle"),
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
            render_layers: data.render_layers,
            index: AnimIndex {
                body_type,
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0 / data.fps,
                next,
            },
        }
    }
}
