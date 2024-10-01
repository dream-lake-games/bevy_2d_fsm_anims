use bevy::{reflect::Reflect, render::view::RenderLayers};
use bevy_2delight_anims::{derive_anim, prelude::*};
use bevy_2delight_anims_macros::AnimStateMachine;

struct MainLayer;
impl Into<RenderLayers> for MainLayer {
    fn into(self) -> RenderLayers {
        RenderLayers::from_layers(&[0])
    }
}

derive_anim!(
    pub enum CircleAnim {
        #[default]
        #[file("platformer/circle.png")]
        #[size(24, 24)]
        #[length(8)]
        #[fps(3.0)]
        #[zix(10.0)]
        #[render_layers(MainLayer)]
        Spin,
    }
);

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
