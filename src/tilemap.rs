use std::{fs::File, io::{BufReader, BufRead}};

use bevy::prelude::*;

use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, TILE_SIZE, player::Player};

pub struct TileMapPlugin;

//setting up colision
#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(create_simple_map);
    }
}

//functions
fn create_simple_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    ){

    //finding map.txt file
    let file = File::open("assets/map.txt").expect("No map file found");

    //labeling tiles
    let mut tiles = Vec::new();

    for(y, line) in BufReader::new(file).lines().enumerate(){
        
        //making sure the lines are ok
        if let Ok(line) = line {

            for(x,char) in line.chars().enumerate() {

                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9,0.9,0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0)
                );

                //check and set terms for tiles
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }

                tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}