use crate::Point;
use crate::Screen;
use crate::Vertex;

pub struct Atlas {
    screen: Screen,
}

impl Atlas {
    pub fn new(screen: Screen) -> Atlas {
        Atlas { screen }
    }

    pub fn vertex(&self, point: Point) -> Vertex {
        let screen = &self.screen;
        let (x, y) = from_lng_lat(point);
        screen.percent_to_pixels(x, y)
    }
}

fn mercator_x_from_lng(lng: f64) -> f64 {
    (lng + 180.0) / 360.0
}

fn mercator_y_from_lat(lat: f64) -> f64 {
    let sin_lat = lat.to_radians().sin();
    0.5 - ((1.0 + sin_lat) / (1.0 - sin_lat)).log(std::f64::consts::E)
        / (4.0 * std::f64::consts::PI)
}

fn from_lng_lat(point: Point) -> (f64, f64) {
    (
        mercator_x_from_lng(point.lng),
        mercator_y_from_lat(point.lat),
    )
}
