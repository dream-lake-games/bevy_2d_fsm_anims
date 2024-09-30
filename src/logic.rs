use std::marker::PhantomData;

use bevy::prelude::*;

use crate::body::BodyState;
use crate::man::{AnimMan, AnimResetStateInfo};
use crate::mat::AnimMat;
use crate::traits::AnimStateMachine;
use crate::{AnimIxChange, AnimNextState, AnimSet, AnimStateChange};

/// Placed on components which need to mutably access some material of theirs this frame in response to state of ix changes.
#[derive(Component, Default)]
struct NeedsMatStateUpdate<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}

/// Placed on components which need to mutably access some material of theirs this frame in response to flip changes.
#[derive(Component, Default)]
struct NeedsMatFlipUpdate<StateMachine: AnimStateMachine> {
    _pd: PhantomData<StateMachine>,
}

/// For animations which are not already marked as having a `reset_state` this frame,
/// play them by incrementing time and (potentially) moving forward ixes and states.
/// Regardless, after this frame, any animation which has a non-None `reset_state` will
/// also have a `NeedsMatStateUpdate` component attached to it. This allows us to avoid
/// traversing the list of all animation again.
fn progress_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>)>,
    mut bodies: Query<&mut BodyState<StateMachine>>,
    time: Res<Time>,
) {
    for (anim_eid, mut anim_man) in &mut anims {
        // If the reset_state is not None, it means `.reset_state` has been called.
        // This state should take precedence over any state/ix that would arise from
        // just naturally playing animations
        if anim_man.reset_state.is_none() {
            // Initialize
            let mut despawned_or_removed = false;
            let initial_state = anim_man.state;
            let mut current_state = initial_state;
            let mut current_body = bodies
                .get(anim_man.tagged_children[&current_state])
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
                                .get(anim_man.tagged_children[&current_state])
                                .expect("Anim missing body2");
                        }
                        AnimNextState::Despawn => {
                            commands.entity(anim_eid).despawn_recursive();
                            despawned_or_removed = true;
                        }
                        AnimNextState::Remove => {
                            for body in anim_man.tagged_children.values() {
                                commands.entity(*body).despawn_recursive();
                            }
                            commands.entity(anim_eid).remove::<AnimMan<StateMachine>>();
                            despawned_or_removed = true;
                        }
                    }
                }
            }
            // If we haven't despawned, do some updating
            if !despawned_or_removed {
                if current_state != initial_state || current_ix != initial_ix {
                    // A state transition is happening
                    anim_man.reset_state = Some(AnimResetStateInfo {
                        state: current_state,
                        ix: current_ix,
                        time: current_time,
                    });
                } else {
                    // Otherwise, we need to increment the time of the current body
                    let mut current_body = bodies
                        .get_mut(anim_man.tagged_children[&current_state])
                        .expect("ANim missing body3");
                    current_body.time = current_time;
                }
            }
        }
        // Make sure it has `NeedsMatStateUpdate` if it has some reset_state
        if anim_man.reset_state.is_some() {
            commands
                .entity(anim_eid)
                .insert(NeedsMatStateUpdate::<StateMachine>::default());
        }
        // Make sure it has `NeedsMatFlipUpdate` if it has reset_flip
        if anim_man.reset_flip {
            commands
                .entity(anim_eid)
                .insert(NeedsMatFlipUpdate::<StateMachine>::default());
        }
    }
}

fn drive_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>), With<NeedsMatStateUpdate<StateMachine>>>,
    mut bodies: Query<(
        &mut BodyState<StateMachine>,
        &mut Visibility,
        &Handle<AnimMat>,
    )>,
    mut mats: ResMut<Assets<AnimMat>>,
) {
    for (eid, mut anim_man) in &mut anims {
        let reset = anim_man.reset_state.as_ref().expect(
            "having NeedsMatStateUpdate should imply reset_state.is_some() by time drive_animations runs",
        );
        if reset.state != anim_man.state {
            // Hide and reset the last body when changing states
            let (mut old_body, mut old_vis, old_hand) = bodies
                .get_mut(anim_man.tagged_children[&anim_man.state])
                .expect("tagged_children looks off in drive_animations1");
            old_body.ix = 0;
            *old_vis = Visibility::Hidden;
            let old_mat = mats
                .get_mut(old_hand.id())
                .expect("drive_animations: bodies should have AnimMats1");
            old_mat.set_ix(old_body.ix);
            // Trigger a change if being observed
            if anim_man.observe_state_changes {
                commands.trigger(AnimStateChange {
                    prev: Some(anim_man.state),
                    next: reset.state,
                });
            }
        }
        // NEW BODY RHUMBA
        let (mut new_body, mut new_vis, new_hand) = bodies
            .get_mut(anim_man.tagged_children[&reset.state])
            .expect("tagged_children looks off in drive_animations1");
        new_body.ix = reset.ix;
        new_body.time = reset.time;
        *new_vis = Visibility::Inherited;
        let new_mat = mats
            .get_mut(new_hand.id())
            .expect("drive_animations: bodies should have AnimMats1");
        new_mat.set_ix(new_body.ix);
        new_mat.set_flip_x(anim_man.flip_x);
        new_mat.set_flip_y(anim_man.flip_y);
        // TODO(maybe): If the game gets really laggy, it may progress more than one ix per frame
        // Right now, this will only trigger once. It'd be better to trigger on every missed ix.
        if anim_man.observe_ix_changes {
            commands.trigger(AnimIxChange {
                state: reset.state,
                ix: reset.ix,
            });
        }
        // Cleanup
        anim_man.state = reset.state;
        anim_man.reset_state = None;
        commands
            .entity(eid)
            .remove::<NeedsMatStateUpdate<StateMachine>>();
    }
}

fn drive_flips<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut anims: Query<(Entity, &mut AnimMan<StateMachine>), With<NeedsMatFlipUpdate<StateMachine>>>,
    bodies: Query<&Handle<AnimMat>, With<BodyState<StateMachine>>>,
    mut mats: ResMut<Assets<AnimMat>>,
) {
    for (eid, mut anim_man) in &mut anims {
        let hand = bodies
            .get(anim_man.tagged_children[&anim_man.state])
            .expect("");
        let mat = mats
            .get_mut(hand.id())
            .expect("drive_animations: bodies should have AnimMats1");
        mat.set_flip_x(anim_man.flip_x);
        mat.set_flip_y(anim_man.flip_y);
        // Cleanup
        anim_man.reset_flip = false;
        commands
            .entity(eid)
            .remove::<NeedsMatFlipUpdate<StateMachine>>();
    }
}

pub(crate) fn register_logic<StateMachine: AnimStateMachine>(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            progress_animations::<StateMachine>,
            drive_animations::<StateMachine>,
            drive_flips::<StateMachine>,
        )
            .chain()
            .in_set(AnimSet),
    );
}
