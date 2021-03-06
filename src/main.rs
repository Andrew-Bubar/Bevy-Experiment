#![allow(clippy::redundant_field_names)] //allowing double naming in the main file

use bevy::{prelude::*, render::camera::ScalingMode};

mod player;
mod debug;
mod ascii;
mod tilemap;

use debug::DebugPlugin;
use player::PlayerPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main(){

    let height = 900.0;

    App::new()

    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        width: height * RESOLUTION,
        height: height,
        title: "Bevy Test".to_string(),
        vsync: true,
        resizable: false,
        ..Default::default()
    })

    //start systems
    .add_startup_system(spawn_camera)

    .add_plugins(DefaultPlugins)

    //my plugins
    .add_plugin(PlayerPlugin)
    .add_plugin(DebugPlugin)
    .add_plugin(AsciiPlugin)
    .add_plugin(TileMapPlugin)

    .run();
}

fn spawn_camera(mut commands: Commands){

    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

