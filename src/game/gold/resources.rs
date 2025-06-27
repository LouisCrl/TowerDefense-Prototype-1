//the resources.rs file is for the resources of the gold used in systems.rs
use bevy::prelude::*;

//the Gold resource is a value of golds
#[derive(Resource)]
pub struct Gold {
    pub value: usize,
}

impl Default for Gold {
    fn default() -> Gold {
        Gold { value: 10 }
    }
}