use bevy::prelude::*;

use crate::{arena::Arena, player::Player};
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_empty_game).run();
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Game {
    arena: Option<Arena>,
    player1: Option<Player>,
    player2: Option<Player>,
}

fn start_empty_game(mut commands: Commands) {
    commands.spawn(Game {
        arena: None,
        player1: None,
        player2: None,
    });
}
