mod arena;
mod common;
mod player;
mod senzubean;
use arena::ArenaPlugin;
use bevy::prelude::*;
use player::PlayerPlugin;
use senzubean::SenzubeanPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(SenzubeanPlugin)
        .run();
}
