use avian3d::prelude::*;
use bevy::prelude::*;

use crate::CharacterOnGround;

use super::components::{CharacterGround, CharacterGroundSensor};

pub fn process_ground_start_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<CharacterGroundSensor>, Without<CharacterOnGround>)>,
    query_ground: Query<Entity, With<CharacterGround>>,
    mut collision_start_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(c1, c2) in collision_start_event_reader.read() {
        if query_groundsensor.contains(*c1) {
            if query_ground.contains(*c2) {
                commands.entity(*c1).insert(CharacterOnGround);
            }
        } else if query_groundsensor.contains(*c2) {
            if query_ground.contains(*c1) {
                commands.entity(*c2).insert(CharacterOnGround);
            }
        }
    }
}

pub fn process_ground_end_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<CharacterGroundSensor>, With<CharacterOnGround>)>,
    query_ground: Query<Entity, With<CharacterGround>>,
    mut collision_ended_event_reader: EventReader<CollisionEnded>,
) {
    for CollisionEnded(c1, c2) in collision_ended_event_reader.read() {
        if query_groundsensor.contains(*c1) {
            if query_ground.contains(*c2) {
                commands.entity(*c1).remove::<CharacterOnGround>();
            }
        } else if query_groundsensor.contains(*c2) {
            if query_ground.contains(*c1) {
                commands.entity(*c2).remove::<CharacterOnGround>();
            }
        }
    }
}
