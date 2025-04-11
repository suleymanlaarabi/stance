use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterMovementSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterCameraSet;
