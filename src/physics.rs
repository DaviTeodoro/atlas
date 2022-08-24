use specs::prelude::*;

use crate::components::*;

struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Up => pos.0 = pos.0.offset(0, -vel.speed),
                Down => pos.0 = pos.0.offset(0, vel.speed),
                Right => pos.0 = pos.0.offset(vel.speed, 0),
                Left => pos.0 = pos.0.offset(-vel.speed, 0),
            }
        }
    }
}
