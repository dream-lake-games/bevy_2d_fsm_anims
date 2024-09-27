use anims::{LennyAnim, SpaceShipAnimPlugin};
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::WindowResolution};
use bevy_2d_fsm_anims::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod anims;

fn main() {
    let mut app = App::new();

    // Bevy (or ecosystem) Plugins
    use bevy::asset::AssetMetaCheck;
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    title: "PLATFORMER".to_string(),
                    resolution: WindowResolution::new(320.0 * 3.0, 180.0 * 3.0),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)));

    // Anim plugins
    app.add_plugins(AnimPlugin::default());
    app.add_plugins(SpaceShipAnimPlugin::default());

    app.add_systems(Startup, startup);

    // Have fun!
    app.run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("camera"), Camera2dBundle::default()));
    commands.spawn((Name::new("sanity_sprite"), SpriteBundle::default()));
    commands.spawn((
        Name::new("lenny"),
        AnimMan::<LennyAnim>::new(),
        SpatialBundle::from_transform(Transform::from_scale(Vec3::ONE * 4.0)),
    ));
}
