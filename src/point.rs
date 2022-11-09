use crate::Vertex;
use std::f64::consts::{E, PI};

const DEG_TO_RAD: f32 = (PI / 180.0) as f32;
const RAD_TO_DEG: f32 = (180.0 / PI) as f32;

#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub lat: f32,
    pub lng: f32,
}

impl Point {
    pub fn new(lat: f32, lng: f32) -> Self {
        Point { lat, lng }
    }

    pub fn get_screen_space_pos(&self, screen_height: f32) -> Vertex {
        let (x, y) = mercator_from_lng_lat(self);
        Vertex {
            x: (x * screen_height),
            y: (y * screen_height),
        }
    }
    pub fn get_tile(&self, zoom: i32) -> (i32, i32, i32) {
        let sin = (self.lat * DEG_TO_RAD).sin();
        let zoom_squared = zoom.pow(2) as f32;
        let mut x = zoom_squared * (self.lng / 360. + 0.5);
        let y = zoom_squared * (0.5 - 0.25 * ((1. + sin) / (1. - sin)).log(E as f32) / PI as f32);
        x = x % zoom_squared;
        if x < 0. {
            x = x + zoom_squared;
        }
        (x as i32, y as i32, zoom)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lng)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lng)
    }
}

impl Into<Point> for geo_types::Coordinate<f32> {
    fn into(self) -> Point {
        Point {
            lng: self.x,
            lat: self.y,
        }
    }
}
impl Into<Point> for (f32, f32) {
    fn into(self) -> Point {
        Point::new(self.1, self.0)
    }
}
impl Into<Point> for (f64, f64) {
    fn into(self) -> Point {
        Point::new(self.1 as f32, self.0 as f32)
    }
}

impl Into<Point> for geo_types::Coordinate {
    fn into(self) -> Point {
        Point::new(self.y as f32, self.x as f32)
    }
}
//

pub fn mercator_from_lng_lat(point: &Point) -> (f32, f32) {
    (
        -1. + (mercator_x_from_lng(point.lng) * 2.),
        1. - (mercator_y_from_lat(point.lat) * 2.),
    )
}

pub fn mercator_into_lng_lat(x: f32, y: f32) -> Point {
    let x = (x + 1.) / 2.;
    let y = (1. - y) / 2.;
    Point::new(lat_from_mercator_y(y), lng_from_mercator_x(x))
}

fn mercator_x_from_lng(lng: f32) -> f32 {
    (lng + 180.0) / 360.0
}

fn mercator_y_from_lat(lat: f32) -> f32 {
    let sin_lat = lat.to_radians().sin();
    0.5 - ((1.0 + sin_lat) / (1.0 - sin_lat)).log(E as f32) / (4. * PI as f32)
}

fn lat_from_mercator_y(y: f32) -> f32 {
    let y2 = 180.0 - y * 360.0;
    360.0 / PI as f32 * ((y2 * PI as f32 / 180.0).exp()).atan() - 90.0
}

fn lng_from_mercator_x(x: f32) -> f32 {
    x * 360.0 - 180.0
}
