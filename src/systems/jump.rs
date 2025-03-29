use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use gravix::components::OnGround;

use crate::components::CharacterInputConfig;

pub fn handle_jump(
    mut query: Query<(&mut LinearVelocity, &CharacterInputConfig), With<OnGround>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, config) in &mut query {
        if keys.just_pressed(config.jump) {
            velocity.y += 24.;
        }
    }
}
