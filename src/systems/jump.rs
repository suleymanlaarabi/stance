use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{CharacterOnGround, components::CharacterInputConfig};

pub fn handle_jump(
    mut query: Query<(&mut LinearVelocity, &CharacterInputConfig), With<CharacterOnGround>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, config) in &mut query {
        if keys.just_pressed(config.jump) {
            velocity.y += 24.;
        }
    }
}
