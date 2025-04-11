use bevy::prelude::*;

pub mod components;
mod macros;
pub mod prelude;
pub mod state;
mod systems;

use state::{CharacterCameraState, CharacterMovementState, CharacterState};
use systems::{
    camera::rotate_player_camera,
    jump::handle_jump,
    movement::move_player,
    set::{CharacterCameraSet, CharacterMovementSet},
};

pub struct VeloxCharacterPlugin;

impl Plugin for VeloxCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CharacterMovementState::Enabled);
        app.insert_state(CharacterCameraState::Enabled);
        app.insert_state(CharacterState::Enabled);
        let movement_set = (move_player, handle_jump)
            .run_if(in_state(CharacterMovementState::Enabled))
            .in_set(CharacterMovementSet);

        let camera_set = rotate_player_camera
            .run_if(in_state(CharacterCameraState::Enabled))
            .in_set(CharacterCameraSet);

        let update_systems = (movement_set, camera_set).run_if(in_state(CharacterState::Enabled));

        app.add_systems(Update, update_systems);
    }
}
