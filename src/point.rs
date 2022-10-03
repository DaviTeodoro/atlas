#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
}

impl Point {
    pub fn new(lat: f64, lng: f64) -> Self {
        Point { lat, lng }
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

impl Into<Point> for (f64, f64) {
    fn into(self) -> Point {
        Point::new(self.1, self.0)
    }
}
impl Into<Point> for geo_types::Coordinate {
    fn into(self) -> Point {
        Point::new(self.y, self.x)
    }
}
//
