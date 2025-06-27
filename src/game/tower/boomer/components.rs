//the components.rs file is for the components concerning boomers
use bevy::prelude::*;

//the Boomer component here to get the stats of the boomer tower
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Boomer {
    pub attack_damage: usize,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub damage_radius: f32,
    pub around_aoe: bool,
}

impl Default for Boomer {
    fn default() -> Self {
        Boomer {
            attack_damage: 3,
            attack_range: 3.0,
            attack_speed: 0.7,
            damage_radius: 3.0,
            around_aoe: false,
        }
    }
}