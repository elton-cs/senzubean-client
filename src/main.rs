mod arena;
mod common;
mod game;
mod player;
mod senzubean;
mod tilemap;
use arena::ArenaPlugin;
use bevy::prelude::*;
use player::PlayerPlugin;
use senzubean::SenzubeanPlugin;
use tilemap::TilePlugin;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(SenzubeanPlugin)
        .add_plugins(TilePlugin)
        .add_systems(Startup, spawn_camera_setup)
        .run();
}

fn spawn_camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
