use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs_derive::Component;

use crate::{vertex::Vertex, ScrollDirection};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct MouseControlled;

/// The current geometry of a shape
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Geometry(pub Vec<Vec<Vertex>>);

/// The current camera state
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Camera {
    x: i32,
    y: i32,
    drag_start_x: i32,
    drag_start_y: i32,
    pub scale: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    is_dragging: bool,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0,
            y: 0,
            drag_start_x: 0,
            drag_start_y: 0,
            scale: 1.0,
            delta_x: 0.,
            delta_y: 0.,
            offset_x: 0.,
            offset_y: 0.,
            is_dragging: false,
        }
    }
    pub fn move_to(&mut self, x: i32, y: i32) {
        if self.is_dragging {
            self.x = x;
            self.y = y;
            self.delta_x = (self.x - self.drag_start_x) as f32 / self.scale;
            self.delta_y = (self.y - self.drag_start_y) as f32 / self.scale;
            self.offset_x += self.delta_x;
            self.offset_y += self.delta_y;
            self.drag_start_x = self.x;
            self.drag_start_y = self.y;
        } else {
            self.set_drag_start(x, y)
        }
    }
    pub fn set_drag_start(&mut self, x: i32, y: i32) {
        self.is_dragging = true;
        self.drag_start_x = x;
        self.drag_start_y = y;
    }
    pub fn set_drag_end(&mut self) {
        self.drag_start_x = 0;
        self.drag_start_y = 0;
        self.is_dragging = false;
    }

    pub fn zoom_to(&mut self, x: i32, y: i32, direction: ScrollDirection) {
        let zoom_factor = match direction {
            ScrollDirection::Up => 0.2 * self.scale,
            ScrollDirection::Down => -0.2 * self.scale,
        };

        let before_zoom_x = (x as f32 / self.scale) - self.offset_x as f32;
        let before_zoom_y = (y as f32 / self.scale) - self.offset_y as f32;

        self.scale += zoom_factor;

        let after_zoom_x = (x as f32 / self.scale) - self.offset_x as f32;
        let after_zoom_y = (y as f32 / self.scale) - self.offset_y as f32;

        self.offset_x += after_zoom_x - before_zoom_x;
        self.offset_y += after_zoom_y - before_zoom_y;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: i16,
    pub y: i16,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
