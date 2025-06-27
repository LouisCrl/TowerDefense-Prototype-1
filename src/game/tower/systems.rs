//the systems.rs file is for the systems of the tower using resources in resources.rs
use super::archer::components::Archer;
use super::knight::components::Knight;
use super::boomer::components::Boomer;
use super::components::*;
use super::resources::*;
use super::events::*;

use crate::game::enemy::components::{Enemy, Path, MarkedForDespawn};
use crate::game::gold::resources::*;
use crate::game::resources::Map;

use bevy::window::PrimaryWindow;
use bevy::input::ButtonInput;
use bevy::prelude::*;

const DISPLAY_TILE_SIZE: f32 = 32.0;

//the spawn_text_entity system is to spawn the text indicating the tower we are currently placing
pub fn spawn_text_entity (
    mut commands: Commands,
) {
    commands.spawn((
        Text2d::new(' '.to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Transform {
            translation: Vec3::new(-610.0, 330.0, 3.0),
            ..default()
        },
        CurrentTextEntity,
    ));
}

//the spawn_tower system is here to spawn the tower with a certain type and the coordonates pointed by the mouse
pub fn spawn_tower(
    (x ,y): (f32, f32),
    asset_server: &AssetServer,
    commands: &mut Commands,
    tower_type: TowerType,
) {
    let sprite = match tower_type {
        TowerType::Archer(_) => asset_server.load("sprites/archer.png"),
        TowerType::Knight(_) => asset_server.load("sprites/knight.png"),
        TowerType::Boomer(_) => asset_server.load("sprites/boomer.png"),
    };

    commands.spawn((
        Sprite {
            image: sprite,
            custom_size: Some(Vec2::splat(16.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 2.0),
        Tower { 
            tower_type,
            coor: (x, y),
            cooldown: 0.0,
        },
    ));

    let range_in_px = tower_type.get_stats().1 * DISPLAY_TILE_SIZE;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            custom_size: Some(Vec2::splat(range_in_px * 2.0)),
            color: Color::srgba(0.0, 0.0, 1.0, 0.3),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        GlobalTransform::default(), // Obligatoire dans Bevy 0.16
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

//the check_condition_tower system here to verify if the current coordonates are available to place a tower
pub fn check_condition_tower(
    (world_x, world_y): (f32, f32),
    tower_query: &Query<&Tower>,
    map_resource: &Res<Map>,
) -> bool {
    let world_top_left_x = - (map_resource.map[0].len() as f32 * DISPLAY_TILE_SIZE) / 2.0;
    let world_top_left_y = (map_resource.map.len() as f32 * DISPLAY_TILE_SIZE) / 2.0;

    if world_x < world_top_left_x || world_y > world_top_left_y {
        info!("Position hors des limites du monde : ({}, {})", world_x, world_y);
        return false;
    }

    let tile_x = ((world_x - world_top_left_x) / DISPLAY_TILE_SIZE).floor() as usize;
    let tile_y = ((world_top_left_y - world_y) / DISPLAY_TILE_SIZE).floor() as usize;

    if tile_y >= map_resource.map.len() || tile_x >= map_resource.map[0].len() {
        info!("Position hors de la map : ({}, {})", tile_x, tile_y);
        return false;
    }

    let tile = map_resource.map[tile_y][tile_x];
    if tile != 7 && tile != 8 {
        info!("Tuile non valide ({}) à ({}, {})", tile, tile_x, tile_y);
        return false;
    }

    let occupied = tower_query.iter().any(|tower| {
        let (tx, ty) = tower.coor;
        let tile_tx = ((tx - world_top_left_x) / DISPLAY_TILE_SIZE).floor();
        let tile_ty = ((world_top_left_y - ty) / DISPLAY_TILE_SIZE).floor();
        tile_tx == tile_x as f32 && tile_ty == tile_y as f32
    });

    if occupied {
        info!("Une tour est déjà présente à la tuile ({}, {})", tile_x, tile_y);
    }

    !occupied
}

//the on_mouse_click system is the system we do when the mouse is clicked, here to invocate a tower
pub fn on_mouse_click (
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut placement_event_writer: EventWriter<TowerPlacementRequest>,
    mut text_query: Query<&mut Text2d, With<CurrentTextEntity>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut placement_state : ResMut<PlacementState>,
    buttons: Res<ButtonInput<MouseButton>>,
    map_resource: Res<Map>,
) {
    if buttons.just_pressed(MouseButton::Left) {

        let tower: TowerType;

        match *placement_state {
            PlacementState { archer: true, boomer: false, knight: false } => tower = TowerType::Archer(Archer::default()),
            PlacementState { archer: false, boomer: true, knight: false } => tower = TowerType::Boomer(Boomer::default()),
            PlacementState { archer: false, boomer: false, knight: true } => tower = TowerType::Knight(Knight::default()),
            PlacementState { archer: false, boomer: false, knight: false } => return,
            _ => return,
        };

        let Ok((camera, camera_transform)) = camera_query.single() else { return; };
        let window = windows.single();

        if let Some(cursor_position_screen) = window.unwrap().cursor_position() {
            if let Ok(cursor_position_world) = camera.viewport_to_world_2d(camera_transform, cursor_position_screen) {
                let world_top_left_x = - (map_resource.map[0].len() as f32 * DISPLAY_TILE_SIZE) / 2.0;
                let world_top_left_y = (map_resource.map.len() as f32 * DISPLAY_TILE_SIZE) / 2.0;

                // Conversion en indices avec décalage
                let tile_x = ((cursor_position_world.x - world_top_left_x) / DISPLAY_TILE_SIZE).floor();
                let tile_y = ((world_top_left_y - cursor_position_world.y) / DISPLAY_TILE_SIZE).floor();

                let aligned = Vec2::new(tile_x, tile_y);

                placement_event_writer.write(TowerPlacementRequest {
                    position: aligned,
                    tower_type: tower,
                });

                placement_state.archer = false;
                placement_state.boomer = false;
                placement_state.knight = false;

                if let Ok(mut current_text) = text_query.single_mut() {
                    current_text.0 = ' '.to_string();
                }
            }
        }
    }
}

//the on_keyboard_key_pressed system is here when the keyboard is clicked to prepare the tower spawn
pub fn on_keyboard_key_pressed (
    mut text_query: Query<&mut Text2d, With<CurrentTextEntity>>,
    mut placement_state : ResMut<PlacementState>,
    buttons: Res<ButtonInput<KeyCode>>,
    golds: Res<Gold>,
) {
    if let Ok(mut current_text) = text_query.single_mut() {
        if golds.value >= 10 {
            let mut new_text= current_text.0.clone();

            if buttons.just_pressed(KeyCode::Digit1) {
                placement_state.archer = true;
                placement_state.boomer = false;
                placement_state.knight = false;
                new_text = 'A'.to_string();
            }
            if buttons.just_pressed(KeyCode::Digit2) {
                placement_state.archer = false;
                placement_state.boomer = true;
                placement_state.knight = false;
                new_text = 'B'.to_string();
            }
            if buttons.just_pressed(KeyCode::Digit3) {
                placement_state.archer = false;
                placement_state.boomer = false;
                placement_state.knight = true;
                new_text = 'K'.to_string();
            }
            if buttons.just_pressed(KeyCode::Escape) {
                placement_state.archer = false;
                placement_state.boomer = false;
                placement_state.knight = false;
                new_text = ' '.to_string();
            }

            current_text.0 = new_text;
        }
    }
}

//the handle_tower_placement system here to handle the tower placement with the great coordonates
pub fn handle_tower_placement(
    mut event_reader: EventReader<TowerPlacementRequest>,
    asset_server: Res<AssetServer>,
    tower_query: Query<&Tower>,
    mut golds: ResMut<Gold>,
    map_resource: Res<Map>,
    mut commands: Commands,
) {
    let world_top_left_x = - (map_resource.map[0].len() as f32 * DISPLAY_TILE_SIZE) / 2.0;
    let world_top_left_y = (map_resource.map.len() as f32 * DISPLAY_TILE_SIZE) / 2.0;

    for event in event_reader.read() {
        let world_pos_x = event.position.x * DISPLAY_TILE_SIZE + world_top_left_x + DISPLAY_TILE_SIZE / 2.0;
        let world_pos_y = world_top_left_y - event.position.y * DISPLAY_TILE_SIZE - DISPLAY_TILE_SIZE / 2.0;

        if check_condition_tower(
            (world_pos_x, world_pos_y),
            &tower_query,
            &map_resource
        ) {
            spawn_tower(
                (world_pos_x, world_pos_y),
                &asset_server,
                &mut commands,
                event.tower_type,
            );
            golds.value -= 10;
        }
    }
}

//the tower_attack system is here to make the towers attacks the most advanced enemy after the cooldown of the tower passed
pub fn tower_attack(
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy, &Path), With<Enemy>>,
    mut tower_query: Query<(&Transform, &mut Tower)>,
    mut golds: ResMut<Gold>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (tower_transform, mut tower) in tower_query.iter_mut() {
        tower.cooldown -= time.delta_secs();
        if tower.cooldown > 0.0 {
            continue;
        }

        let tower_pos = tower_transform.translation.truncate();
        let (damage, range, speed, damage_radius, around_aoe) = tower.tower_type.get_stats();

        let range_in_pixel = range * DISPLAY_TILE_SIZE;
        let damage_radius_in_pixel = damage_radius * DISPLAY_TILE_SIZE;

        let mut attacked = false;

        let mut enemies_in_range: Vec<(Entity, Vec2, usize)> = enemy_query.iter()
            .filter_map(|(entity, transform, _enemy, path)| {
                let enemy_pos = transform.translation.truncate();
                let distance = tower_pos.distance(enemy_pos);
                if distance <= range_in_pixel {
                    Some((entity, enemy_pos, path.current))
                } else {
                    None
                }
            })
            .collect();

        enemies_in_range.sort_by(|a, b| b.2.cmp(&a.2));

        for (enemy_entity, enemy_pos, _path) in enemies_in_range.iter() {
            if let Ok((_entity, _transform, mut enemy, _path)) = enemy_query.get_mut(*enemy_entity) {
                if enemy.health <= damage {
                    golds.value += enemy.health;
                    commands.entity(*enemy_entity).insert(MarkedForDespawn);
                } else {
                    golds.value += damage;
                    enemy.health -= damage;
                }
                attacked = true;

                for (other_enemy_entity, other_enemy_transform, mut other_enemy, _path) in enemy_query.iter_mut() {
                    let other_enemy_pos = other_enemy_transform.translation.truncate();
                    let dist_to_explosion = enemy_pos.distance(other_enemy_pos);

                    if dist_to_explosion <= damage_radius_in_pixel && other_enemy_entity != *enemy_entity {
                        if other_enemy.health <= damage {
                            golds.value += other_enemy.health;
                            commands.entity(other_enemy_entity).insert(MarkedForDespawn);
                        } else {
                            golds.value += damage;
                            other_enemy.health -= damage;
                        }
                    }
                }

                if !around_aoe {
                    tower.cooldown = 1.0 / speed;
                    break;
                }
            }
        }

        if attacked {
            tower.cooldown = 1.0 / speed;
        }
    }
}