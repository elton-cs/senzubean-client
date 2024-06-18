use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
struct Arena {
    starting_bounds: Vec2,
    ending_bounds: Vec2,
}
