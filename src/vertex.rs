#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}
impl Into<Vertex> for (f32, f32) {
    fn into(self) -> Vertex {
        Vertex {
            x: self.0 as i16,
            y: self.1 as i16,
        }
    }
}
