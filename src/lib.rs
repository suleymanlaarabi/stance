use bevy::prelude::*;

pub mod components;
pub mod prelude;
mod systems;

use systems::movement::move_player;

pub struct VeloxCharacterPlugin;

impl Plugin for VeloxCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
