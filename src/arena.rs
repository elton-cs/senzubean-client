use crate::{
    common::Point2D,
    tilemap::{ArenaMapAtlas, TileMapAtlas},
};
use bevy::prelude::*;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_arena)
            .add_systems(PostStartup, display_arenas)
            .add_systems(Update, render_arena);
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Arena {
    starting_bounds: Point2D,
    ending_bounds: Point2D,
}

impl Arena {
    pub fn new(x: u32, y: u32) -> Arena {
        Arena {
            starting_bounds: Point2D::new(0, 0),
            ending_bounds: Point2D::new(x, y),
        }
    }
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

fn render_arena(
    mut commands: Commands,
    query_atlas: Query<&mut TileMapAtlas, With<ArenaMapAtlas>>,
    query_arena: Query<&Arena>,
) {
    let atlas = query_atlas.single();
    let texture = atlas.iamge_handle.clone();

    let top_left = 61;
    let top_edge = 62;
    let top_right = 63;

    let left_edge = 73;
    let center = 74;
    let right_edge = 75;

    let bottom_left = 85;
    let bottom_edge = 86;
    let bottom_right = 87;

    let texture_atlas = TextureAtlas {
        layout: atlas.atlas_layout_handle.clone(),
        index: bottom_left,
    };

    let arena = query_arena.single();

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    // Transform::from_scale(Vec3::splat(4.0))

    for i in 0..arena.ending_bounds.x {
        for j in 0..arena.ending_bounds.y {
            let index = match (i, j) {
                (0, 0) => bottom_left,
                (0, y) if y == arena.ending_bounds.y - 1 => top_left,
                (x, 0) if x == arena.ending_bounds.x - 1 => bottom_right,
                (x, y) if x == arena.ending_bounds.x - 1 && y == arena.ending_bounds.y - 1 => {
                    top_right
                }
                (0, _) => left_edge,
                (_, 0) => bottom_edge,
                (x, _) if x == arena.ending_bounds.x - 1 => right_edge,
                (_, y) if y == arena.ending_bounds.y - 1 => top_edge,
                _ => center,
            };

            let texture_atlas = TextureAtlas {
                layout: atlas.atlas_layout_handle.clone(),
                index,
            };

            commands.spawn((
                SpriteBundle {
                    transform: transform
                        .with_translation(Vec3::new(16.0 * i as f32, 16.0 * j as f32, 0.0))
                        .with_scale(Vec3::splat(1.0)),
                    texture: texture.clone(),
                    ..default()
                },
                texture_atlas.clone(),
            ));
        }
        // commands.spawn((
        //     SpriteBundle {
        //         transform: transform.with_translation(Vec3::new(0. + 16.0 * i as f32, 0.0, 0.0)),
        //         texture: texture.clone(),
        //         ..default()
        //     },
        //     texture_atlas.clone(),
        // ));
    }

    // commands.spawn((
    //     SpriteBundle {
    //         transform: transform,
    //         texture: texture.clone(),
    //         ..default()
    //     },
    //     texture_atlas.clone(),
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         transform: transform.with_translation(Vec3::new(16.0, 0.0, 0.0)),
    //         texture,
    //         ..default()
    //     },
    //     texture_atlas.clone(),
    // ));
}
