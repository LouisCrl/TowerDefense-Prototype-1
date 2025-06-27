//the events.rs file is here to handle the potential event of the tower systems
use super::components::*;

use bevy::prelude::*;

//the TowerPlacementRequest is an event when we want to place a tower with a position and a type
#[derive(Event)]
pub struct TowerPlacementRequest {
    pub position: Vec2,
    pub tower_type: TowerType,
}