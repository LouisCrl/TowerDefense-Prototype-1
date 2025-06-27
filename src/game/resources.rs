//the resources.rs file is for the resources of the game used in systems.rs
use bevy::prelude::*;

//the Map resource is a 40*24 board wich is the map
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub struct Map
{
    pub map: [[usize; 40]; 24],
}

impl Default for Map {
    fn default() -> Map {
        Map {
            map: [[0; 40]; 24],
        }
    }
}