use bevy::prelude::*;
use systems::*;

pub(crate) mod components;
pub(crate) mod systems;

#[derive(Default)]
pub struct CharacterGroundedPlugin;

impl Plugin for CharacterGroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (process_ground_end_sensor, process_ground_start_sensor),
        );
    }
}
