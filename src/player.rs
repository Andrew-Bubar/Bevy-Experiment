
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{ascii::{AsciiPlugin, AsciiSheet, spawn_ascii_sprite}, TILE_SIZE, tilemap::TileCollider};

pub struct PlayerPlugin;

//making the player more special
#[derive(Component, Inspectable)]
pub struct Player{
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

//functions
fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    col_query: Query<&Transform, (With<TileCollider>, Without<Player>)>, 
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    //making the player be both a sprite and transform
    let(_player, mut transform) = player_query.single_mut();

    //inputs
    let mut y_delta = 0.0; //getting the y axis input
    if keyboard.pressed(KeyCode::W) | keyboard.pressed(KeyCode::Up){
        y_delta += _player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) | keyboard.pressed(KeyCode::Down) {
        y_delta -= _player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0; //getting the x axis input
    if keyboard.pressed(KeyCode::A) | keyboard.pressed(KeyCode::Left) {
        x_delta -= _player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) | keyboard.pressed(KeyCode::Right) {
        x_delta += _player.speed * TILE_SIZE * time.delta_seconds();
    }

    //applying the movement with collision
    let target = transform.translation + Vec3::new(x_delta, 0.0,0.0);
    if wall_col_check(target, &col_query){
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta,0.0);
    if wall_col_check(target, &col_query){
        transform.translation = target;
    }
}

fn wall_col_check(
    target_player_pos: Vec3,
    col_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>, 
) -> bool {
    for wall_transform in col_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE)
        );

        if collision.is_some(){
            return false;
        }
    }
    return true;
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){

    //player ascii entity
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3,0.3,0.9),
        Vec3::new(2.0 * TILE_SIZE,-2.0 * TILE_SIZE,900.0),
    );

    //commands for player spawn
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player{ speed: 3.0})
        .id();

        //player ascii entity
        let background = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            0,
            Color::rgb(0.5,0.5,0.5),
            Vec3::new(0.0,0.0,-1.0),
        );
    
        //commands for player spawn
        commands
            .entity(background)
            .insert(Name::new("Background"))
            .id();

    //making the background a child of the player
    commands.entity(player) /* .push_children(&[background]) */;
}