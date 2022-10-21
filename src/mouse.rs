use specs::prelude::*;

use crate::{components::*, Cursor};

use super::MouseCommand;

pub struct Mouse;

impl<'a> System<'a> for Mouse {
    type SystemData = (
        ReadExpect<'a, Cursor>,
        ReadStorage<'a, MouseControlled>,
        WriteStorage<'a, Velocity>,
        WriteExpect<'a, Camera>,
    );
    fn run(&mut self, mut data: Self::SystemData) {
        let cursor = &*data.0;
        let camera = &mut data.3;
        let mouse_command = cursor.command;
        let position = (cursor.x, cursor.y);

        match mouse_command {
            MouseCommand::Click => {
                camera.set_drag_start(position.0, position.1);
            }
            MouseCommand::Hold => {
                camera.move_to(position.0, position.1);
            }
            MouseCommand::Release => camera.set_drag_end(),
            MouseCommand::Scroll(direction) => camera.zoom_to(position.0, position.1, direction),
            MouseCommand::None => {}
        };
    }
}
