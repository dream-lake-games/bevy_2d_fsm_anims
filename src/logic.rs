use bevy::prelude::*;

use crate::body::BodyIndex;
use crate::man::AnimMan;
use crate::state::AnimState;
use crate::traits::{AnimStateMachine, ManageAnims};
use crate::AnimNextState;

/// Marks an animation that would progress normally this frame (if uninterupted)
#[derive(Component)]
struct ProgressUpdate<StateMachine: AnimStateMachine> {
    state: StateMachine,
    ix: u32,
    time: f32,
}
/// Finds all animations that would have natural progressions this frame, and inserts progress updates
fn create_progress_updates<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    anims: Query<(Entity, &AnimState<StateMachine>)>,
    bodies: Query<&BodyIndex<StateMachine>>,
    time: Res<Time>,
) {
    for (anim_eid, anim_state) in &anims {
        // Initialize
        let mut despawned = false;
        let initial_state = anim_state.state;
        let mut current_state = initial_state;
        let mut current_body = bodies
            .get(anim_state.tagged_children[&current_state])
            .expect("Anim missing body1");
        let mut current_time = current_body.time;
        let initial_ix = current_body.ix;
        let mut current_ix = initial_ix;
        // Transition through ixs and states
        current_time += time.delta_seconds();
        while current_time > current_body.spf {
            current_ix += 1;
            current_time -= current_body.spf;
            if current_ix >= current_body.length {
                match current_body.next {
                    AnimNextState::Stay => {
                        current_ix = 0;
                    }
                    AnimNextState::Some(next_state) => {
                        current_state = next_state;
                        current_ix = 0;
                        current_body = bodies
                            .get(anim_state.tagged_children[&current_state])
                            .expect("Anim missing body2");
                    }
                    AnimNextState::Despawn => {
                        commands.entity(anim_eid).despawn_recursive();
                        despawned = true;
                    }
                }
            }
        }
        // If we haven't despawned and would change anims
        if !despawned && (current_state != initial_state || current_ix != initial_ix) {
            commands.entity(anim_eid).insert(ProgressUpdate {
                state: current_state,
                ix: current_ix,
                time: current_time,
            });
        }
    }
}

fn progress_mutate_anim_mans<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    already_changed: Query<Entity, Changed<AnimMan<StateMachine>>>,
    mut anim_q: Query<(
        Entity,
        &ProgressUpdate<StateMachine>,
        &AnimState<StateMachine>,
        &mut AnimMan<StateMachine>,
    )>,
    mut body_q: Query<&mut BodyIndex<StateMachine>>,
) {
    for (eid, update, anim_state, mut anim_man) in &mut anim_q {
        if !already_changed.contains(eid) {
            let is_state_change = update.state != anim_state.state;
            if is_state_change {
                anim_man.reset_state(update.state);
            } else {
                let body = body_q
                    .get_mut(anim_state.tagged_children[&anim_state.state])
                    .expect("mutate_anim_mans1");
            }
        }
        commands
            .entity(eid)
            .remove::<ProgressUpdate<StateMachine>>();
    }
}

// fn handle_manager_changes<StateMachine: AnimStateMachine>(
//     mut commands: Commands,
//     mans: Query<
//         (Entity, &AnimMan<StateMachine>, Option<&Children>),
//         Changed<AnimMan<StateMachine>>,
//     >,
//     relevant_children: Query<Entity, With<AnimIndex<StateMachine>>>,
//     ass: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut mats: ResMut<Assets<AnimMat>>,
//     anim_defaults: Res<AnimDefaults>,
// ) {
//     for (eid, man, ochildren) in &mans {
//         if let Some(children) = ochildren {
//             for child in children {
//                 if relevant_children.contains(*child) {
//                     commands.entity(*child).despawn_recursive();
//                 }
//             }
//         }
//         let state = man.get_state();
//         let data = state.get_data();
//         let next = state.get_next().clone();
//         let body_bund = AnimBodyBundle::new(
//             state,
//             data,
//             next,
//             man.flip_x,
//             man.flip_y,
//             &ass,
//             &mut meshes,
//             &mut mats,
//             &anim_defaults,
//         );
//         commands.spawn(body_bund).set_parent(eid);
//         if man.observe_state_changes {
//             commands.trigger_targets(
//                 AnimStateChange {
//                     prev: man.last_state,
//                     state: man.state,
//                 },
//                 eid,
//             );
//         }
//     }
// }

// fn play_animations<StateMachine: AnimStateMachine>(
//     mut commands: Commands,
//     mut managers: Query<(Entity, &mut AnimMan<StateMachine>)>,
//     mut bodies: Query<(&mut AnimIndex<StateMachine>, &Handle<AnimMat>, &Parent)>,
//     mut mats: ResMut<Assets<AnimMat>>,
//     time: Res<Time>,
// ) {
//     for (mut index, hand, parent) in &mut bodies {
//         let Ok((manager_eid, mut manager)) = managers.get_mut(parent.get()) else {
//             continue;
//         };
//         index.time += time.delta_seconds();
//         let mut despawning = false;
//         while index.spf < index.time && !despawning {
//             index.ix += 1;
//             index.time -= index.spf;
//             if index.length <= index.ix {
//                 // This specific animation variant has finished
//                 match index.next {
//                     AnimNextState::Stay => {}
//                     AnimNextState::Some(next_state) => {}
//                     AnimNextState::Despawn => {
//                         commands.entity(manager_eid).despawn_recursive();
//                         despawning = true;
//                     }
//                 }
//             }
//         }
//         if index.length <= index.ix {
//             // This specific animation variant has finished
//             match index.next {
//                 AnimNextState::Stay => {
//                     index.ix = 0;
//                 }
//                 AnimNextState::Some(next_state) => {}
//                 AnimNextState::Despawn => {
//                     commands.entity(manager_eid).despawn_recursive();
//                     despawning = true;
//                 }
//             }
//         }
//     }

//     // for (mut index, hand, parent) in &mut bodies {
//     //     let Ok((manager_eid, mut manager, mut visibility)) = managers.get_mut(parent.get()) else {
//     //         continue;
//     //     };
//     //     index.time += time.delta_seconds();
//     //     if index.time < index.spf {
//     //         // No update is happening to this body, can just continue
//     //         continue;
//     //     }
//     //     index.time = 0.0;
//     //     if index.ix + 1 < index.length {
//     //         // Progressing to the next frame of the animation
//     //         index.ix += 1;
//     //         let mat = mats.get_mut(hand.id()).unwrap();
//     //         mat.set_ix(index.ix);
//     //     } else {
//     //         match &index.next {
//     //             AnimNextState::Stay => {
//     //                 // Looping the animation
//     //                 if index.length <= 1 {
//     //                     // Degen animations don't need to do anything
//     //                     continue;
//     //                 }
//     //                 index.ix = 0;
//     //                 let mat = mats.get_mut(hand.id()).unwrap();
//     //                 mat.set_ix(index.ix);
//     //             }
//     //             AnimNextState::Some(variant) => {
//     //                 // Transitioning to a new state
//     //                 manager.reset_state(variant.clone());
//     //             }
//     //             AnimNextState::Despawn => {
//     //                 // Triggering the death process for this entity
//     //                 *visibility = Visibility::Hidden;
//     //                 commands.entity(manager_eid).despawn_recursive();
//     //             }
//     //         }
//     //     }
//     // }
// }

// #[derive(Component)]
// struct AnimPopulatedCache;
// fn populate_caches<StateMachine: AnimStateMachine>(
//     ass: Res<AssetServer>,
//     mut anim_man_q: Query<(Entity, &mut AnimMan<StateMachine>), Without<AnimPopulatedCache>>,
//     mut commands: Commands,
// ) {
//     for (eid, mut anim_man) in &mut anim_man_q {
//         for state in StateMachine::iter() {
//             anim_man
//                 .handle_cache
//                 .insert(state, ass.load(state.get_data().get_path()));
//             commands.entity(eid).insert(AnimPopulatedCache);
//         }
//     }
// }

pub(crate) fn register_logic<StateMachine: AnimStateMachine>(_app: &mut App) {
    // app.add_systems(
    //     PostUpdate,
    //     (
    //         handle_manager_changes::<StateMachine>,
    //         play_animations::<StateMachine>,
    //         populate_caches::<StateMachine>,
    //     )
    //         .chain()
    //         .in_set(AnimSet),
    // );
}
