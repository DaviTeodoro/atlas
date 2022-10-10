use specs::prelude::*;

use crate::components::*;
use crate::vertex::Vertex;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Geometry>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (geometry, vel) in (&mut data.0, &data.1).join() {
            geometry.0 = geometry
                .0
                .iter()
                .map(|shape| {
                    shape
                        .iter()
                        .map(|v| Vertex {
                            x: v.x + vel.x,
                            y: v.y + vel.y,
                        })
                        .collect()
                })
                .collect();
        }
    }
}
