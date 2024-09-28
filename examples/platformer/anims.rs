use bevy::reflect::Reflect;
use bevy_2d_fsm_anims::prelude::*;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, EnumIter)]
pub enum CircleAnim {
    #[default]
    Spin,
}
impl AnimStateMachine for CircleAnim {
    fn get_data(&self) -> AnimBody {
        match self {
            Self::Spin => AnimBody::new("platformer/circle.png", 24, 24).with_length(8),
        }
    }
    fn get_next(&self) -> AnimNextState<Self> {
        match self {
            Self::Spin => AnimNextState::Stay,
        }
    }
}
pub type CircleAnimPlugin = AnimDefnPlugin<CircleAnim>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, EnumIter)]
pub enum LennyAnim {
    Idle,
    #[default]
    Run,
    RunOffset,
}
impl AnimStateMachine for LennyAnim {
    fn get_data(&self) -> AnimBody {
        match self {
            Self::Idle => {
                AnimBody::new("platformer/lenny_idle.png", 17, 17).with_offset(-0.5, -0.5)
            }
            Self::Run => AnimBody::new("platformer/lenny_run.png", 17, 17)
                .with_length(5)
                .with_offset(-10.5, -0.5),
            Self::RunOffset => AnimBody::new("platformer/lenny_run.png", 17, 17)
                .with_length(5)
                .with_offset(10.5, -0.5),
        }
    }

    fn get_next(&self) -> AnimNextState<Self> {
        match self {
            Self::Idle => AnimNextState::Stay,
            Self::Run => AnimNextState::Some(LennyAnim::RunOffset),
            Self::RunOffset => AnimNextState::Some(LennyAnim::Run),
        }
    }
}
pub type LennyAnimPlugin = AnimDefnPlugin<LennyAnim>;
