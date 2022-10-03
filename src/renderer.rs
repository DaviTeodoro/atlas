use crate::vertex::Vertex;
use sdl2::pixels;
use sdl2::render::WindowCanvas;

use sdl2::gfx::primitives::DrawRenderer;

pub fn render(geometry_list: &Vec<Vec<Vertex>>, canvas: &mut WindowCanvas) {
    geometry_list.iter().for_each(|geometry| {
        let (vx, vy) = geometry.iter().fold((vec![], vec![]), |acc, vertex| {
            (
                [acc.0, vec![vertex.x]].concat(),
                [acc.1, vec![vertex.y]].concat(),
            )
        });
        canvas
            .filled_polygon(&vx, &vy, pixels::Color::RGB(171, 191, 218))
            .expect("failed to draw triangle");
    });
    canvas.present();
}
