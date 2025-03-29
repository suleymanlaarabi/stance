use bevy::prelude::*;

pub mod components;
pub mod plugins;
pub mod prelude;
mod systems;

use plugins::grounded_plugin::CharacterGroundedPlugin;
pub use plugins::grounded_plugin::components::*;
use systems::{camera::rotate_player_camera, jump::handle_jump, movement::move_player};

pub struct VeloxCharacterPlugin;

impl Plugin for VeloxCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterGroundedPlugin);
        app.add_systems(Update, (move_player, rotate_player_camera, handle_jump));
    }
}
