//the components.rs file is for the components concerning archers
use bevy::prelude::*;

//the Archer component here to get the stats of the archer tower
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Archer {
    pub attack_damage: usize,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub damage_radius: f32,
    pub around_aoe: bool,
}

impl Default for Archer {
    fn default() -> Self {
        Archer {
            attack_damage: 1,
            attack_range: 5.0,
            attack_speed: 1.5,
            damage_radius: 0.0,
            around_aoe: false,
        }
    }
}