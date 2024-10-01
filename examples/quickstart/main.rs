use bevy::{prelude::*, render::view::RenderLayers};
use bevy_2delight_anims::prelude::*;

struct MainLayer;
impl Into<RenderLayers> for MainLayer {
    fn into(self) -> RenderLayers {
        RenderLayers::from_layers(&[0])
    }
}

// You also could write:
// #[derive(Debug, Copy, Clone, Default, Reflect, PartialEq, Eq, Hash, AnimStateMachine)]
// but that's a lot to remember.
derive_anim!(
    pub enum CircleAnim {
        #[default]
        // The file containing the animation. There should be one file per animation, with frames arranged horizontally
        #[file("platformer/circle.png")]
        // How big is a single frame of the animation?
        #[size(24, 24)]
        // How many frames are in the animation?
        // OPTIONAL: Defaults to 1
        #[length(8)]
        // What is the FPS of this animation?
        // OPTIONAL: Defaults to the value in `AnimPlugin`.
        #[fps(3.0)]
        // Should the animation be offset from it's parent?
        // OPTIONAL: Defaults to (0.0, 0.0)
        #[offset(10.0, 0.0)]
        // What should the z-index of this animation be?
        // OPTIONAL: Defaults to 0.0
        // NOTE: If you have multiple animations on an entity and notice flickering/overlapping, it's likely because
        //       they all have the same zix. Give them an explicit ordering.
        #[zix(10.0)]
        // What render layers should this animation show up in?
        // OPTIONAL: Defaults to the value in `AnimPlugin`.
        // NOTE: You must provide a list of identifiers, each of which implements `Into<RenderLayers>`.
        //       The final `RenderLayers` that will be attached to the animation is the union of what you provide.
        #[render_layers(MainLayer)]
        // After this animation completes, what should happen?
        // OPTIONAL: Defaults to looping. You can provide the name of another variant (like 'Spin'), Despawn, or Remove.
        #[next(Despawn)]
        Spin,
    }
);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(AnimPlugin {
        default_fps: 16.0,
        default_render_layers: RenderLayers::default(),
    });
    app.add_plugins(AnimDefnPlugin::<CircleAnim>::default());

    app.add_systems(Startup, startup);

    app.run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("camera"), Camera2dBundle::default()));
    commands.spawn((
        Name::new("circle"),
        AnimMan::<CircleAnim>::default(),
        SpatialBundle::from_transform(Transform::from_scale(Vec3::ONE * 6.0)),
    ));
}
