use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[macro_use]
extern crate load_file;
mod camera;
mod features;
mod point;
mod vertex;

use camera::LoadCamera;
use features::LoadFeatures;
use point::Point;
use vertex::Vertex;

// /// This system prints out all mouse events as they come in
// fn print_mouse_events_system(
//     mut mouse_button_input_events: EventReader<MouseButtonInput>,
//     mut mouse_motion_events: EventReader<MouseMotion>,
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mut mouse_wheel_events: EventReader<MouseWheel>,
// ) {
//     for event in mouse_button_input_events.iter() {
//         // info!("{:?}", event);
//     }

//     for event in mouse_motion_events.iter() {
//         // info!("{:?}", event);
//     }

//     for event in cursor_moved_events.iter() {
//         // info!("{:?}", event);
//     }

//     for event in mouse_wheel_events.iter() {
//         // info!("{:?}", event);
//     }
// }
//

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadCamera)
        .add_plugin(ShapePlugin)
        .add_plugin(LoadFeatures)
        .run();
}
