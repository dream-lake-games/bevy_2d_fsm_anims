use bevy::ecs::world::DeferredWorld;
use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};

use crate::lazy::{impl_get_ref, impl_with, impl_with_option};
use crate::mat::AnimMat;
use crate::plugin::AnimDefaults;
use crate::traits::AnimStateMachine;
use crate::AnimNextState;

/// Core data defining a single animation
#[derive(Debug, Clone, Reflect)]
pub struct AnimBody {
    path: String,
    size: UVec2,
    length: u32,
    fps: Option<f32>,
    offset: Vec2,
    zix: f32,
    render_layers: Option<RenderLayers>,
}

impl AnimBody {
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

/// Used internally for tracking animations that play
#[derive(Component, Debug, Clone, Reflect)]
pub(crate) struct BodyState<StateMachine: AnimStateMachine> {
    pub(crate) ix: u32,
    pub(crate) length: u32,
    pub(crate) time: f32,
    /// Seconds per frame
    pub(crate) spf: f32,
    /// The state to transition to after this state. Note that this has a None variant inside it.
    pub(crate) next: AnimNextState<StateMachine>,
}

#[derive(Bundle, Clone)]
pub(crate) struct AnimBodyBundle<StateMachine: AnimStateMachine> {
    name: Name,
    mesh: Mesh2dHandle,
    material: Handle<AnimMat>,
    spatial: SpatialBundle,
    render_layers: RenderLayers,
    index: BodyState<StateMachine>,
}
impl<StateMachine: AnimStateMachine> AnimBodyBundle<StateMachine> {
    pub(crate) fn new(
        state: StateMachine,
        flip_x: bool,
        flip_y: bool,
        visible: bool,
        world: &mut DeferredWorld,
    ) -> Self {
        let data = state.get_data();
        let next = state.get_next();
        let mesh = Mesh::from(Rectangle::new(data.size.x as f32, data.size.y as f32));
        let texture = world.resource::<AssetServer>().load(data.path);
        Self {
            name: Name::new(format!("AnimBody_{state:?}")),
            mesh: world.resource_mut::<Assets<Mesh>>().add(mesh).into(),
            material: world.resource_mut::<Assets<AnimMat>>().add(AnimMat::new(
                texture,
                data.length,
                flip_x,
                flip_y,
            )),
            spatial: SpatialBundle {
                transform: Transform {
                    translation: data.offset.extend(data.zix),
                    ..default()
                },
                visibility: if visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
                ..default()
            },
            render_layers: data.render_layers.unwrap_or(
                world
                    .resource::<AnimDefaults>()
                    .default_render_layers
                    .clone(),
            ),
            index: BodyState {
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0
                    / data
                        .fps
                        .unwrap_or(world.resource::<AnimDefaults>().default_fps),
                next,
            },
        }
    }
}
