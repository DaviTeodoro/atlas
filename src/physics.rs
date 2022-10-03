use specs::prelude::*;

use crate::components::*;
use crate::vertex::Vertex;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Geometry>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        for (geometry, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Down => {
                    geometry.0 = geometry
                        .0
                        .iter()
                        .map(|shape| {
                            shape
                                .iter()
                                .map(|v| Vertex {
                                    x: v.x,
                                    y: v.y + vel.speed as i16,
                                })
                                .collect()
                        })
                        .collect()
                }
                Up => {
                    geometry.0 = geometry
                        .0
                        .iter()
                        .map(|shape| {
                            shape
                                .iter()
                                .map(|v| Vertex {
                                    x: v.x,
                                    y: v.y - vel.speed as i16,
                                })
                                .collect()
                        })
                        .collect()
                }
                Left => {
                    geometry.0 = geometry
                        .0
                        .iter()
                        .map(|shape| {
                            shape
                                .iter()
                                .map(|v| Vertex {
                                    x: v.x - vel.speed as i16,
                                    y: v.y,
                                })
                                .collect()
                        })
                        .collect()
                }
                Right => {
                    geometry.0 = geometry
                        .0
                        .iter()
                        .map(|shape| {
                            shape
                                .iter()
                                .map(|v| Vertex {
                                    x: v.x + vel.speed as i16,
                                    y: v.y,
                                })
                                .collect()
                        })
                        .collect()
                }
            }
        }
    }
}
