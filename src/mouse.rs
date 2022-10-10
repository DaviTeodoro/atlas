use specs::prelude::*;

use crate::components::*;

use super::MouseCommand;

pub struct Mouse;

impl<'a> System<'a> for Mouse {
    type SystemData = (
        ReadExpect<'a, MouseCommand>,
        ReadStorage<'a, MouseControlled>,
        WriteStorage<'a, Velocity>,
        WriteExpect<'a, Camera>,
    );
    fn run(&mut self, mut data: Self::SystemData) {
        let mouse_command = &*data.0;
        let camera = &mut data.3;

        match mouse_command {
            MouseCommand::Click(position) => {
                camera.set_drag_start(position.0, position.1);
            }
            MouseCommand::Hold(position) => {
                camera.move_to(position.0, position.1);
            }
            MouseCommand::Release(_) => camera.set_drag_end(),
            MouseCommand::Scroll(_) => {}
            MouseCommand::None => {}
        };
    }
}
