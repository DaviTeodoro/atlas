use bevy::prelude::Vec2;

use crate::point::{mercator_into_lng_lat, Point};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}
impl Into<Vertex> for (f32, f32) {
    fn into(self) -> Vertex {
        Vertex {
            x: self.0,
            y: self.1,
        }
    }
}

impl Vertex {
    pub fn into_point(&self, screen_size: Vec2) -> Point {
        let (_, height) = (screen_size.x, screen_size.y);
        let x = self.x / height;
        let y = self.y / height;
        mercator_into_lng_lat(x, y)
    }
}
