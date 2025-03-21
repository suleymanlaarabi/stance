use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Collider, CollidingEntities)]
pub struct CharacterGroundSensor;

#[derive(Component)]
pub struct CharacterGround;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct CharacterOnGround;
