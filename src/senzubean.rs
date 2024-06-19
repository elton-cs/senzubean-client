use crate::common::Point2D;
use bevy::prelude::*;

pub struct SenzubeanPlugin;
impl Plugin for SenzubeanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_senzubean)
            .add_systems(PostStartup, display_senzubean);
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Senzubean {
    vec: Point2D,
    is_eaten: bool,
}

fn spawn_senzubean(mut commands: Commands) {
    commands.spawn(Senzubean {
        vec: Point2D::new(6, 3),
        is_eaten: false,
    });
}

fn display_senzubean(query: Query<(Entity, &Senzubean)>) {
    for (entity, senzubean) in query.iter() {
        info!("Entity {:?}: {:?}", entity, senzubean);
    }
}
