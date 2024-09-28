use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

use crate::body::AnimBodyBundle;
use crate::lazy::impl_get_copy;
use crate::traits::AnimStateMachine;

/// The current animation state.
/// NOTE: Should only be used to READ animation state. To change state, use `AnimMan`.
#[derive(Debug, Clone, Reflect)]
pub struct AnimState<StateMachine: AnimStateMachine> {
    /// Current state of the animation
    pub(crate) state: StateMachine,
    /// Last state of the animation
    pub(crate) last_state: Option<StateMachine>,
    /// Is the animation flipped in the x direction?
    pub(crate) flip_x: bool,
    /// Is the animation flipped in the y direction?
    pub(crate) flip_y: bool,
    /// Should the `AnimStateChange` event be triggered?
    pub(crate) observe_state_changes: bool,
    /// Should the `AnimIxChange` event be triggered?
    pub(crate) observe_ix_changes: bool,
    /// INTERNAL: The entities of the spawned body children
    pub(crate) tagged_children: HashMap<StateMachine, Entity>,
}
impl<StateMachine: AnimStateMachine> AnimState<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            last_state: None,
            flip_x: false,
            flip_y: false,
            observe_state_changes: false,
            observe_ix_changes: false,
            tagged_children: default(),
        }
    }

    impl_get_copy!(state, StateMachine);
    impl_get_copy!(flip_x, bool);
    impl_get_copy!(flip_y, bool);
}
impl<StateMachine: AnimStateMachine> Component for AnimState<StateMachine> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let myself = world
                .get::<Self>(eid)
                .expect("AnimState: on_add hook should have myself");
            let flip_x = myself.flip_x;
            let flip_y = myself.flip_y;
            let my_state = myself.state;
            let mut tagged_children = HashMap::default();
            for state in StateMachine::iter() {
                let bund =
                    AnimBodyBundle::new(state, flip_x, flip_y, state == my_state, &mut world);
                let mut commands = world.commands();
                let mut ent_comms = commands.spawn(bund);
                ent_comms.set_parent(eid);
                let child_eid = ent_comms.id();
                tagged_children.insert(state, child_eid);
            }
        });
    }
}
