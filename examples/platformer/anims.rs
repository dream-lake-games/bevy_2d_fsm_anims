use bevy::{reflect::Reflect, render::view::RenderLayers};
use bevy_2delight_anims::{derive_anim, prelude::*};
use bevy_2delight_anims_macros::AnimStateMachine;

struct MainLayer;
impl Into<RenderLayers> for MainLayer {
    fn into(self) -> RenderLayers {
        RenderLayers::from_layers(&[1])
    }
}

struct OtherLayer;
impl Into<RenderLayers> for OtherLayer {
    fn into(self) -> RenderLayers {
        RenderLayers::from_layers(&[2])
    }
}

#[derive(Debug, Copy, Clone, Default, Reflect, PartialEq, Eq, Hash, AnimStateMachine)]
pub enum CircleAnim {
    #[default]
    #[file("platformer/circle.png")]
    #[size(24, 24)]
    #[length(8)]
    #[fps(1.0)]
    #[zix(10.0)]
    #[next(Remove)]
    Spin,
}

derive_anim!(
    pub enum LennyAnim {
        #[file("platformer/lenny_idle.png")]
        #[size(17, 17)]
        #[offset_x_negative]
        #[offset_y_negative]
        #[offset(0.5, 0.5)]
        Idle,

        #[default]
        #[file("platformer/lenny_run.png")]
        #[size(17, 17)]
        #[length(5)]
        #[offset_x_negative]
        #[offset_y_negative]
        #[offset(10.5, 10.5)]
        #[next(RunOffset)]
        Run,

        #[file("platformer/lenny_run.png")]
        #[size(17, 17)]
        #[length(5)]
        #[offset(10.5, 10.5)]
        #[next(Run)]
        RunOffset,
    }
);
