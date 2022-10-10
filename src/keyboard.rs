use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;

const SPEED: i16 = 5;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );
    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => match direction {
                    Direction::Up => {
                        vel.y = -SPEED;
                    }
                    Direction::Down => {
                        vel.y = SPEED;
                    }
                    Direction::Left => {
                        vel.x = -SPEED;
                    }
                    Direction::Right => {
                        vel.x = SPEED;
                    }
                },
                MovementCommand::Stop => {
                    vel.x = 0;
                    vel.y = 0
                }
            };
        }
    }
}
