use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::body::AnimBodyDataBundle;
use crate::man::AnimMan;
use crate::mat::AnimMat;
use crate::traits::{AnimBody, AnimStateMachine, ManageAnims};
use crate::{AnimBodyProgress, AnimIndex, AnimNextState, AnimSet};

fn handle_manager_changes<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    managers: Query<
        (Entity, &AnimMan<StateMachine>, Option<&Children>),
        Changed<AnimMan<StateMachine>>,
    >,
    relevant_children: Query<Entity, With<AnimIndex<StateMachine>>>,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<AnimMat>>,
) {
    for (eid, manager, ochildren) in &managers {
        if let Some(children) = ochildren {
            for child in children {
                if relevant_children.contains(*child) {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
        let mut new_progress_map = HashMap::new();
        let state_data = manager.get_state().to_state_data();
        for (ix, body) in StateMachine::BodyType::all_bodies().into_iter().enumerate() {
            new_progress_map.insert(body, 0);
            let data = body.to_body_data();
            let next = if ix == 0 {
                state_data.clone()
            } else {
                AnimNextState::None
            };
            let body_bund = AnimBodyDataBundle::new(
                body,
                data,
                next,
                &ass,
                &mut meshes,
                &mut mats,
                manager.flip_x,
                manager.flip_y,
            );
            commands.spawn(body_bund).set_parent(eid);
        }
        commands
            .entity(eid)
            .insert(AnimBodyProgress::<StateMachine> {
                ixes: new_progress_map,
            });
    }
}

fn play_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut managers: Query<(
        Entity,
        &mut AnimMan<StateMachine>,
        &mut AnimBodyProgress<StateMachine>,
        &mut Visibility,
    )>,
    mut bodies: Query<(&mut AnimIndex<StateMachine>, &Handle<AnimMat>, &Parent)>,
    mut mats: ResMut<Assets<AnimMat>>,
    time: Res<Time>,
) {
    for (mut index, hand, parent) in &mut bodies {
        let (manager_eid, mut manager, mut progress, mut visibility) =
            managers.get_mut(parent.get()).unwrap();
        if manager.hidden {
            continue;
        }
        index.time += time.delta_seconds();
        if index.time < index.spf {
            // No update is happening to this body, can just continue
            continue;
        }
        index.time = 0.0;
        if index.ix + 1 < index.length {
            // Progressing to the next frame of the animation
            index.ix += 1;
            let mat = mats.get_mut(hand.id()).unwrap();
            mat.set_ix(index.ix);
        } else {
            match &index.next {
                AnimNextState::None => {
                    // Looping the animation
                    if index.length <= 1 {
                        // Degen animations don't need to do anything
                        continue;
                    }
                    index.ix = 0;
                    let mat = mats.get_mut(hand.id()).unwrap();
                    mat.set_ix(index.ix);
                }
                AnimNextState::Some(variant) => {
                    // Transitioning to a new state
                    manager.reset_state(variant.clone());
                }
                AnimNextState::Despawn => {
                    // Triggering the death process for this entity
                    manager.set_hidden(true);
                    *visibility = Visibility::Hidden;
                    commands.entity(manager_eid).despawn_recursive();
                }
            }
        }
        // Update the ix in the manager so it can be read
        progress.ixes.insert(index.body_type, index.ix);
    }
}

/// I think this is okay? Maybe?
/// Since only one strong handle is ever stored for the same path, this seems fine
/// Has the nice property that once no more entities AnimMan<X> exist, these handles will be dropped
fn populate_caches<StateMachine: AnimStateMachine>(
    ass: Res<AssetServer>,
    mut anim_man_q: Query<&mut AnimMan<StateMachine>>,
) {
    for mut anim_man in &mut anim_man_q {
        for body in StateMachine::BodyType::all_bodies() {
            if anim_man.handle_cache.contains_key(&body) {
                continue;
            }
            anim_man
                .handle_cache
                .insert(body, ass.load(body.to_body_data().path));
        }
    }
}

pub(crate) fn register_logic<StateMachine: AnimStateMachine>(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            handle_manager_changes::<StateMachine>,
            play_animations::<StateMachine>,
            populate_caches::<StateMachine>,
        )
            .chain()
            .in_set(AnimSet),
    );
}
