use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::WindowCanvas;
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = ReadStorage<'a, Geometry>;

pub fn render(data: SystemData, canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.set_draw_color(pixels::Color::RGB(226, 232, 240));
    canvas.clear();
    for geometry in (&data).join() {
        for shape in geometry.0.iter() {
            let (vx, vy) = shape.iter().fold((vec![], vec![]), |acc, vertex| {
                (
                    [acc.0, vec![vertex.x]].concat(),
                    [acc.1, vec![vertex.y]].concat(),
                )
            });
            canvas.filled_polygon(&vx, &vy, pixels::Color::RGB(171, 191, 218))?;
        }
    }
    canvas.present();
    Ok(())
}
