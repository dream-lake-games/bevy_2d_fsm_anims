use bevy::{prelude::Component, reflect::Reflect};
use bevy_2d_fsm_anims::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum LennyBody {
    Circle,
    Idle,
    Run,
}
impl AnimBody for LennyBody {
    fn all_bodies() -> Vec<Self> {
        vec![Self::Circle, Self::Idle, Self::Run]
    }

    fn to_body_data(&self) -> AnimBodyData {
        match self {
            Self::Circle => AnimBodyData::new("platformer/circle.png", 24, 24).with_length(8),
            Self::Idle => {
                AnimBodyData::new("platformer/lenny_idle.png", 17, 17).with_offset(-0.5, -0.5)
            }
            Self::Run => AnimBodyData::new("platformer/lenny_run.png", 17, 17)
                .with_length(5)
                .with_offset(-0.5, -0.5),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum LennyAnim {
    Explode,
    Idle,
    #[default]
    Run,
}
impl AnimStateMachine for LennyAnim {
    type BodyType = LennyBody;

    fn all_states() -> Vec<Self> {
        vec![Self::Idle, Self::Explode]
    }

    fn next(&self) -> AnimNextState<Self> {
        match self {
            Self::Explode => AnimNextState::Despawn,
            Self::Idle => AnimNextState::Stay,
            Self::Run => AnimNextState::Stay,
        }
    }
}
pub type SpaceShipAnimPlugin = AnimDefnPlugin<LennyAnim>;

struct AnimEnteredState<StateMachine: AnimStateMachine, const STATE: i32> {
    state: StateMachine,
}

impl Component for AnimEnteredState<LennyAnim, { LennyAnim::Idle as i32 }> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(_hooks: &mut bevy::ecs::component::ComponentHooks) {}
}
