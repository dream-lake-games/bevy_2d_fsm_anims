use anims::{CircleAnim, CircleAnimPlugin, LennyAnim, LennyAnimPlugin};
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
    app.add_plugins(AnimPlugin {
        default_fps: 4.0,
        ..default()
    });
    app.add_plugins(CircleAnimPlugin::default());
    app.add_plugins(LennyAnimPlugin::default());

    app.add_systems(Startup, startup);
    app.add_systems(Update, flips);
    app.observe(lenny_anim_state_change);

    // Have fun!
    app.run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("camera"), Camera2dBundle::default()));
    commands.spawn((Name::new("sanity_sprite"), SpriteBundle::default()));
    commands.spawn((
        Name::new("lenny"),
        AnimMan::<LennyAnim>::default(),
        SpatialBundle::from_transform(Transform::from_scale(Vec3::ONE * 4.0)),
    ));
}

fn flips(keyboard: Res<ButtonInput<KeyCode>>, mut anims: Query<&mut AnimMan<LennyAnim>>) {
    if keyboard.just_pressed(KeyCode::KeyX) {
        for mut anim in &mut anims {
            let new_val = !anim.get_flip_x();
            anim.set_flip_x(new_val);
        }
    }
    if keyboard.just_pressed(KeyCode::KeyY) {
        for mut anim in &mut anims {
            let new_val = !anim.get_flip_y();
            anim.set_flip_y(new_val);
        }
    }
}

fn lenny_anim_state_change(trigger: Trigger<AnimStateChange<LennyAnim>>) {
    println!("{:?}", trigger.event().next);
}
