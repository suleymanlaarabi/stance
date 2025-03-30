use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use gravix::components::OnGround;

use crate::{components::CharacterInputConfig, prelude::CharacterJump};

pub fn handle_jump(
    mut query: Query<(&mut LinearVelocity, &CharacterInputConfig, &CharacterJump), With<OnGround>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, config, jump) in &mut query {
        if keys.just_pressed(config.jump) {
            velocity.y += jump.strength;
        }
    }
}
