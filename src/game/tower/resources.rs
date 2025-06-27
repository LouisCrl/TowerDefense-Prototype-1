//the resources.rs file is for the resources of the tower used in systems.rs
use bevy::prelude::*;

//the PlacementState resource here to know which tower we are curently placing
#[derive(Resource, Debug, Default)]
pub struct PlacementState {
    pub archer: bool,
    pub boomer: bool,
    pub knight: bool,
}