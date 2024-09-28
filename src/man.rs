use bevy::reflect::Reflect;
use bevy::{prelude::*, utils::HashMap};

use crate::body::AnimBodyBundle;
use crate::lazy::{impl_get_copy, impl_with_on};
use crate::traits::AnimStateMachine;

#[derive(Clone, Debug, Reflect)]
pub(crate) struct AnimResetStateInfo<StateMachine: AnimStateMachine> {
    pub(crate) state: StateMachine,
    pub(crate) ix: u32,
    pub(crate) time: f32,
}

/// The main animation controller
#[derive(Debug, Clone, Reflect)]
pub struct AnimMan<StateMachine: AnimStateMachine> {
    /// Current state of the animation
    pub(crate) state: StateMachine,
    /// Used internally for changing state
    pub(crate) reset_state: Option<AnimResetStateInfo<StateMachine>>,
    /// Used internally for changing flip
    pub(crate) reset_flip: bool,
    /// Flips x-axis of the animation
    pub(crate) flip_x: bool,
    /// Flips y-axis of the animation
    pub(crate) flip_y: bool,
    /// Should the `AnimStateChange` event be triggered?
    pub(crate) observe_state_changes: bool,
    /// Should the `AnimIxChange` event be triggered?
    pub(crate) observe_ix_changes: bool,
    /// INTERNAL: The entities of the spawned body children
    pub(crate) tagged_children: HashMap<StateMachine, Entity>,
}
impl<StateMachine: AnimStateMachine> Default for AnimMan<StateMachine> {
    fn default() -> Self {
        Self {
            state: default(),
            reset_state: Some(AnimResetStateInfo {
                state: default(),
                ix: 0,
                time: 0.0,
            }),
            reset_flip: false,
            flip_x: false,
            flip_y: false,
            observe_state_changes: false,
            observe_ix_changes: false,
            tagged_children: default(),
        }
    }
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn new(state: StateMachine) -> Self {
        Self {
            state,
            reset_state: Some(AnimResetStateInfo {
                state,
                ix: 0,
                time: 0.0,
            }),
            ..default()
        }
    }
    pub fn with_state(mut self, val: StateMachine) -> Self {
        self.state = val;
        self.reset_state.as_mut().unwrap().state = val;
        self
    }
    pub fn with_flip_x(mut self) -> Self {
        self.flip_x = true;
        self.reset_flip = true;
        self
    }
    pub fn with_flip_y(mut self) -> Self {
        self.flip_y = true;
        self.reset_flip = true;
        self
    }
    impl_with_on!(observe_state_changes);
    impl_with_on!(observe_ix_changes);
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    impl_get_copy!(state, StateMachine);
    impl_get_copy!(flip_x, bool);
    impl_get_copy!(flip_y, bool);

    /// If the given state is equal to the current state, nothing happens.
    /// Otherwise, the state is changed to the given state, and the animation is reset to the first frame.
    pub fn set_state(&mut self, state: StateMachine) {
        if self.state != state {
            self.reset_state = Some(AnimResetStateInfo {
                state,
                ix: 0,
                time: 0.0,
            });
        }
    }
    /// The given state is set, and the animation is reset to the first frame.
    pub fn reset_state(&mut self, state: StateMachine) {
        self.reset_state = Some(AnimResetStateInfo {
            state,
            ix: 0,
            time: 0.0,
        });
    }
    /// Set the flipx value of the animation
    pub fn set_flip_x(&mut self, flip_x: bool) {
        if flip_x != self.flip_x {
            self.flip_x = flip_x;
            self.reset_flip = true;
        }
    }
    /// Set the flipy value of the animation
    pub fn set_flip_y(&mut self, flip_y: bool) {
        if flip_y != self.flip_y {
            self.flip_y = flip_y;
            self.reset_flip = true;
        }
    }
}

impl<StateMachine: AnimStateMachine> Component for AnimMan<StateMachine> {
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
            let mut mut_myself = world
                .get_mut::<Self>(eid)
                .expect("ANimState: on_add hook should have myself2");
            mut_myself.tagged_children = tagged_children;
        });
    }
}
