use crate::common::Point2D;
use bevy::prelude::*;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_arena)
            .add_systems(PostStartup, display_arenas);
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Arena {
    starting_bounds: Point2D,
    ending_bounds: Point2D,
}

fn spawn_arena(mut commands: Commands) {
    commands.spawn(Arena {
        starting_bounds: Point2D::new(0, 0),
        ending_bounds: Point2D::new(11, 5),
    });
}

fn display_arenas(query: Query<(Entity, &Arena)>) {
    for (entity, arena) in query.iter() {
        info!("Entity {:?}: {:?}", entity, arena);
    }
}
