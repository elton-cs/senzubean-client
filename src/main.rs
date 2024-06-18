mod arena;
mod player;
mod senzubean;
use bevy::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .run();
}
