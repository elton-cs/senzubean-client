use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
struct Senzubean {
    vec: Vec2,
    is_eaten: bool,
}
