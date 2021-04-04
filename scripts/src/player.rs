// use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

const ACCELERATION: f32 = 500.0;
const FRICTION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;

// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
}

// Player implementation.
#[gdnative::methods]
impl Player {
    // The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        godot_print!("Hello, player");
    }

    //Called every tick that the physics updates.
    //Delta is how long the last frame took - so for 60pfs - delta = 1/60.
    #[export]
    fn _physics_process(&mut self, _owner: &KinematicBody2D, delta: f64) {
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
            self.velocity = self.velocity.move_towards(input_vector * MAX_SPEED, ACCELERATION * delta as f32);
        } else {
            self.velocity = self.velocity.move_towards(Vector2::zero(), FRICTION * delta as f32);
        }

        self.velocity = _owner.move_and_slide(
            self.velocity, 
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }
}
