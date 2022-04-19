
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{ascii::{AsciiPlugin, AsciiSheet, spawn_ascii_sprite}, TILE_SIZE};

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
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    //making the player be both a sprite and transform
    let(_player, mut transform) = player_query.single_mut();

    //inputs
    if keyboard.pressed(KeyCode::W) | keyboard.pressed(KeyCode::Up){
        transform.translation.y += _player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) | keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= _player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) | keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= _player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) | keyboard.pressed(KeyCode::Right) {
        transform.translation.x += _player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){

    //player ascii entity
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3,0.3,0.9),
        Vec3::new(0.0,0.0,900.0),
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
    commands.entity(player).push_children(&[background]);
}