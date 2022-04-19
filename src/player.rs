
use bevy::prelude::*;
use crate::AsciiSheet;

pub struct PlayerPlugin;

//making the player more special
#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app
        .add_startup_system(spawn_player)
        .add_startup_system(player_movement);
    }
}

//functions
fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
){
    //making the player be both a sprite and transform
    let(_player, mut transform) = player_query.single_mut();

    //inputs
    if keyboard.pressed(KeyCode::W){
        transform.translation.y += 0.5;
    }
    if keyboard.pressed(KeyCode::S){
        transform.translation.y -= 0.5;
    }
    if keyboard.pressed(KeyCode::A){
        transform.translation.x -= 0.5;
    }
    if keyboard.pressed(KeyCode::D){
        transform.translation.x += 0.5;
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){

    //player sprite stuff
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3,0.3,0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    //Background player sprite stuff
    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5,0.5,0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));
     
    //sawining in the player sprite
    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(), 
            transform: Transform {
                translation: Vec3::new(0.0,0.0,900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .id();

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(), 
            transform: Transform {
                translation: Vec3::new(0.0,0.0,-1.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Name::new("Background"))
        .id();

    //formating export
    commands.entity(player).push_children(&[background]);
}