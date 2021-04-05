use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

const ACCELERATION: f32 = 500.0;
const FRICTION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;

enum PlayerState {
    MOVE, ROLL, ATTACK
}

// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
    state: PlayerState
}

// Player implementation.
#[gdnative::methods]
impl Player {
    // The "constructor" of the class.
    fn new(owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
            state: PlayerState::MOVE
        }
    }

    #[export]
    fn _ready(&self, owner: &KinematicBody2D) {
        get_typed_node::<AnimationTree, _>("./AnimationTree", owner, |animation_tree| {
            animation_tree.set_active(true);
        });
    }

    //Called every tick that the physics updates.
    //Delta is how long the last frame took - so for 60pfs - delta = 1/60.
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        match self.state {
            PlayerState::MOVE => self.move_state(owner, delta),
            PlayerState::ROLL => {},
            PlayerState::ATTACK => self.attack_state(owner, delta)
        };
    }

    fn move_state(&mut self, owner: &KinematicBody2D, delta: f64) {
        let input = Input::godot_singleton();

        let mut input_vector = Vector2::new(
            Input::get_action_strength(input, "ui_right") as f32
                - Input::get_action_strength(input, "ui_left") as f32,
            Input::get_action_strength(input, "ui_down") as f32
                - Input::get_action_strength(input, "ui_up") as f32,
        );
        input_vector = match Vector2::try_normalize(input_vector) {
            None => Vector2::zero(),
            Some(vector2) => vector2,
        };

        if input_vector != Vector2::zero() {
            get_typed_node::<AnimationTree, _>("./AnimationTree", owner, |animation_tree| {
                animation_tree.set("parameters/Idle/blend_position", input_vector);
                animation_tree.set("parameters/Run/blend_position", input_vector);
                animation_tree.set("parameters/Attack/blend_position", input_vector);

                let animation_state = animation_tree.get("parameters/playback");
                let animation_state = animation_state
                    .try_to_object::<AnimationNodeStateMachinePlayback>()
                    .expect("Should cast to AnimationNodeStateMachinePlayback");
                let animation_state = unsafe { animation_state.assume_safe() };
                animation_state.travel("Run");
            });

            self.velocity = self
                .velocity
                .move_towards(input_vector * MAX_SPEED, ACCELERATION * delta as f32);
        } else {
            get_typed_node::<AnimationTree, _>("./AnimationTree", owner, |animation_tree| {
                let animation_state = animation_tree.get("parameters/playback");
                let animation_state = animation_state
                    .try_to_object::<AnimationNodeStateMachinePlayback>()
                    .expect("Should cast to AnimationNodeStateMachinePlayback");
                let animation_state = unsafe { animation_state.assume_safe() };
                animation_state.travel("Idle");
            });

            self.velocity = self
                .velocity
                .move_towards(Vector2::zero(), FRICTION * delta as f32);
        }

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );

        if input.is_action_just_pressed("attack") {
            self.state = PlayerState::ATTACK;
        }
    }

    fn attack_state(&mut self, owner: &KinematicBody2D, delta: f64) {
        get_typed_node::<AnimationTree, _>("./AnimationTree", owner, |animation_tree| {
            let animation_state = animation_tree.get("parameters/playback");
            let animation_state = animation_state
                .try_to_object::<AnimationNodeStateMachinePlayback>()
                .expect("Should cast to AnimationNodeStateMachinePlayback");
            let animation_state = unsafe { animation_state.assume_safe() };
            animation_state.travel("Attack");
        });
        self.velocity = Vector2::zero();
    }

    #[export]
    fn attack_animation_finished(&mut self, _: &KinematicBody2D) {
        self.state = PlayerState::MOVE;
    }
}
