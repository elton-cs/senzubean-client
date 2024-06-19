use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct Point2D {
    x: u32,
    y: u32,
}

impl Point2D {
    pub fn new(x: u32, y: u32) -> Self {
        Point2D { x, y }
    }
}
