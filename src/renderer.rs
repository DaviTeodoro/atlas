use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::WindowCanvas;
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = (ReadStorage<'a, Geometry>, ReadExpect<'a, Camera>);

pub fn render(data: SystemData, canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.set_draw_color(pixels::Color::RGB(29, 77, 97));
    canvas.clear();
    let camera = &data.1;
    for geometry in (&data.0).join() {
        for shape in geometry.0.iter() {
            let (vx, vy) = shape.iter().fold((vec![], vec![]), |acc, vertex| {
                (
                    [
                        acc.0,
                        vec![((vertex.x as f32 + camera.offset_x) * camera.scale) as i16],
                    ]
                    .concat(),
                    [
                        acc.1,
                        vec![((vertex.y as f32 + camera.offset_y) * camera.scale) as i16],
                    ]
                    .concat(),
                )
            });
            // canvas.filled_polygon(&vx, &vy, pixels::Color::RGB(81, 208, 224))?;
            // canvas.aa_polygon(&vx, &vy, pixels::Color::RGB(49, 135, 145))?;
            canvas.filled_polygon(&vx, &vy, pixels::Color::RGB(61, 208, 178))?;
            canvas.aa_polygon(&vx, &vy, pixels::Color::RGB(64, 150, 129))?;
        }
    }
    canvas.present();
    Ok(())
}
