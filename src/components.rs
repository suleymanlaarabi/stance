use avian3d::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::keyboard::NativeKeyCode,
    prelude::*,
};
use gravix::components::*;

#[derive(Component)]
pub struct CharacterInputConfig {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_front: KeyCode,
    pub move_back: KeyCode,
    pub jump: KeyCode,
    pub dash: KeyCode,
}

const DEFAULT_KEYCODE: KeyCode = KeyCode::Unidentified(NativeKeyCode::Unidentified);

impl CharacterInputConfig {
    pub fn default() -> CharacterInputConfig {
        Self {
            move_back: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_front: KeyCode::KeyW,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            dash: DEFAULT_KEYCODE,
        }
    }
}

#[derive(Component)]
#[require(
    Transform::from_xyz(0., 35., 0.),
    LinearVelocity,
    RigidBody::Dynamic,
    Collider::cuboid(5., 15., 5.),
    LockedAxes::ROTATION_LOCKED,
    GravityScale(5.5),
    GroundSensor
)]
pub struct CharacterController;

#[derive(Component)]
#[require(
    Transform,
    Camera3d,
    Camera = Camera {
        hdr: true,
        ..default()
    },
    Tonemapping::TonyMcMapface,
    Bloom::NATURAL
)]
pub struct CharacterControllerFpsCamera;

#[derive(Component)]
pub struct CharacterMovement {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

#[derive(Component)]
pub struct CharacterAcceleration(pub f32);

impl Default for CharacterMovement {
    fn default() -> Self {
        Self {
            speed: 70.,
            acceleration: 0.,
            deceleration: 0.,
        }
    }
}

#[derive(Component)]
pub struct CharacterJump {
    pub strength: f32,
    pub max_jumps: u8,
    pub count: u8,
}

impl Default for CharacterJump {
    fn default() -> Self {
        Self {
            strength: 24.,
            max_jumps: 1,
            count: 0,
        }
    }
}

#[derive(Component)]
pub struct CharacterDash {
    pub force: f32,
    pub cooldown: f32,
}
