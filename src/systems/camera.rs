use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use std::f32::consts::FRAC_PI_2;

use crate::components::CharacterControllerFpsCamera;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

pub fn rotate_player_camera(
    mut query: Query<&mut Transform, With<CharacterControllerFpsCamera>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = accumulated_mouse_motion.delta * 1.5;

    for mut transform in &mut query {
        if delta != Vec2::ZERO {
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

            transform.rotation = Quat::from_euler(
                EulerRot::YXZ,
                yaw + (-delta.x * 0.001),
                (pitch + (-delta.y * 0.001)).clamp(-PITCH_LIMIT, PITCH_LIMIT),
                roll,
            );
        }
    }
}
