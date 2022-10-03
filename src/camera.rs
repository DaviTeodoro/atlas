#[derive(Debug)]
pub struct Camera {
    x: u32,
    y: u32,
    prev_x: u32,
    prev_y: u32,
    pub delta_x: i32,
    pub delta_y: i32,
    zoom: f64,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32) -> Camera {
        Camera {
            x: screen_width / 2,
            y: screen_height / 2,
            prev_x: screen_width / 2,
            prev_y: screen_height / 2,
            delta_x: 0,
            delta_y: 0,
            zoom: 1.0,
        }
    }
    pub fn move_to(&mut self, x: u32, y: u32) {
        self.prev_x = self.x;
        self.prev_y = self.y;
        self.x = x;
        self.y = y;
        self.delta_x = self.x as i32 - self.prev_x as i32;
        self.delta_y = self.y as i32 - self.prev_y as i32;
    }
}
