//the components.rs file is for the components concerning towers
use super::archer::components::Archer;
use super::boomer::components::Boomer;
use super::knight::components::Knight;
use bevy::prelude::*;

//the Tower component is a tower with a type, a coordonate and a cooldown
#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub coor: (f32, f32),
    pub cooldown: f32,
}

//the TowerType enum which list different types of towers
#[derive(Component, Clone, Copy, PartialEq)]
pub enum TowerType {
    Archer(Archer),
    Knight(Knight),
    Boomer(Boomer),
}

impl TowerType {
    //the get_stats method is to get the different stats of towers depending their types
    pub fn get_stats(&self) -> (usize, f32, f32, f32, bool) {
        match self {
            TowerType::Archer(archer) => (
                archer.attack_damage,
                archer.attack_range,
                archer.attack_speed,
                archer.damage_radius,
                archer.around_aoe,
            ),
            TowerType::Knight(knight) => (
                knight.attack_damage,
                knight.attack_range,
                knight.attack_speed,
                knight.damage_radius,
                knight.around_aoe,
            ),
            TowerType::Boomer(boomer) => (
                boomer.attack_damage,
                boomer.attack_range,
                boomer.attack_speed,
                boomer.damage_radius,
                boomer.around_aoe,
            ),
        }
    }
}

//the CurrentTextEntity component is a text indicating which tower we are currently placing
#[derive(Component)]
pub struct CurrentTextEntity;