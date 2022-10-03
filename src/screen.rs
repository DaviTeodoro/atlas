use crate::Vertex;

pub struct Screen {
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }
    pub fn percent_to_pixels(&self, percent_x: f64, percent_y: f64) -> Vertex {
        let x = self.width;
        let y = self.height;

        Vertex {
            x: (x as f64 * percent_x) as i16,
            y: (y as f64 * percent_y) as i16,
        }
    }
    // fn pixels_to_percent(&self, x: u32, y: u32) -> (f64, f64) {
    //     let width = self.width;
    //     let height = self.height;

    //     ((x as f64 / width as f64), (y as f64 / height as f64))
    // }
}
