use bevy::prelude::*;

use crate::components::*;

const PLAYER_SPEED: f32 = 50.;

pub fn move_player(
    mut query: Query<(&mut Transform, &CharacterInputConfig), With<CharacterController>>,
    camera_query: Query<
        (&ChildOf, &Transform),
        (
            With<CharacterControllerCamera>,
            Without<CharacterController>,
        ),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (parent, camera_transform) in &camera_query {
        let (mut player_transform, config) = match query.get_mut(parent.parent) {
            Ok(transform) => transform,
            Err(_) => {
                continue;
            }
        };
        let mut direction = Vec3::ZERO;
        if keys.pressed(config.move_front) {
            direction += camera_transform.forward().as_vec3();
        }
        if keys.pressed(config.move_back) {
            direction += camera_transform.back().as_vec3();
        }
        if keys.pressed(config.move_left) {
            direction += camera_transform.left().as_vec3();
        }
        if keys.pressed(config.move_right) {
            direction += camera_transform.right().as_vec3();
        }
        if direction != Vec3::ZERO {
            direction.y = 0.;
            let direction = (direction.normalize()) * PLAYER_SPEED * delta_time;
            player_transform.translation.x += direction.x;
            player_transform.translation.z += direction.z;
        }
    }
}
