use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::CharacterOnGround;

use super::components::{CharacterGround, CharacterGroundSensor};

pub(crate) fn process_ground_start_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<CharacterGroundSensor>, Without<CharacterOnGround>)>,
    query_ground: Query<Entity, With<CharacterGround>>,
    mut collision_start_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(c1, c2) in collision_start_event_reader.read() {
        let c1_is_sensor = query_groundsensor.contains(*c1);
        let c2_is_sensor = query_groundsensor.contains(*c2);

        if c1_is_sensor || c2_is_sensor {
            let c1_is_ground = query_ground.contains(*c1);
            let c2_is_ground = query_ground.contains(*c2);

            if (c1_is_sensor && c2_is_ground) || (c2_is_sensor && c1_is_ground) {
                let sensor = if c1_is_sensor { c1 } else { c2 };
                commands.entity(*sensor).insert(CharacterOnGround);
            }
        }
    }
}

pub(crate) fn process_ground_end_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<CharacterGroundSensor>, With<CharacterOnGround>)>,
    query_ground: Query<Entity, With<CharacterGround>>,
    mut collision_ended_event_reader: EventReader<CollisionEnded>,
) {
    for CollisionEnded(c1, c2) in collision_ended_event_reader.read() {
        let c1_is_sensor = query_groundsensor.contains(*c1);
        let c2_is_sensor = query_groundsensor.contains(*c2);

        if c1_is_sensor || c2_is_sensor {
            let c1_is_ground = query_ground.contains(*c1);
            let c2_is_ground = query_ground.contains(*c2);

            if (c1_is_sensor && c2_is_ground) || (c2_is_sensor && c1_is_ground) {
                let sensor = if c1_is_sensor { c1 } else { c2 };
                commands.entity(*sensor).remove::<CharacterOnGround>();
            }
        }
    }
}
