extern crate sdl2;

#[macro_use]
extern crate load_file;

mod atlas;
mod components;
mod keyboard;
mod mouse;
mod physics;
mod point;
mod renderer;
mod screen;
mod vertex;

use atlas::Atlas;
use point::Point;
use screen::Screen;
use vertex::Vertex;

use geo::CoordsIter;

use geo_types::{MultiPolygon, Polygon};
use geojson::{quick_collection, GeoJson, Geometry, Value};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::components::*;
use rayon::prelude::*;

use specs::prelude::*;

// use std::time::Duration;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}
#[derive(Debug, Clone, Copy)]
pub enum MouseCommand {
    Click((i32, i32)),
    Hold((i32, i32, i32, i32)),
    Release((i32, i32)),
    Scroll((i32, i32)),
    None,
}

impl MouseCommand {
    fn update(&self, position: Option<(i32, i32)>) -> MouseCommand {
        match position {
            Some(position) => match self {
                MouseCommand::Click(_) => MouseCommand::Hold((position.0, position.1, 0, 0)),
                MouseCommand::Hold(last_position) => MouseCommand::Hold((
                    position.0,
                    position.1,
                    last_position.0 - position.0,
                    last_position.1 - position.1,
                )),
                MouseCommand::Release(_) => MouseCommand::None,
                MouseCommand::Scroll(_) => MouseCommand::Scroll(position),
                MouseCommand::None => MouseCommand::None,
            },
            None => match self {
                MouseCommand::None | MouseCommand::Scroll(_) => self.to_owned(),

                MouseCommand::Hold(old_position) => {
                    MouseCommand::Hold((old_position.0, old_position.1, 0, 0))
                }
                MouseCommand::Click(old_position) => {
                    MouseCommand::Hold((old_position.0, old_position.1, 0, 0))
                }
                MouseCommand::Release(_) => MouseCommand::None,
            },
        }
    }
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// Process GeoJSON geometries
fn match_geometry(geom: Geometry, world: &mut World, atlas: &Atlas) {
    match geom.value {
        Value::Polygon(_) => {
            let poly: Polygon<f64> = geom.value.try_into().expect("Unable to convert Polygon");
            let geometries = poly
                .coords_iter()
                .map(|c| atlas.vertex(c.into()))
                .collect::<Vec<Vertex>>();
            world
                .create_entity()
                .with(KeyboardControlled)
                .with(MouseControlled)
                .with(Geometry(vec![geometries]))
                .with(Velocity { x: 0, y: 0 })
                .build();
        }
        Value::MultiPolygon(_) => {
            let poly: MultiPolygon<f64> = geom
                .value
                .try_into()
                .expect("Unable to convert MultiPolygon");
            let geometries = poly
                .into_iter()
                .map(|p| {
                    p.coords_iter()
                        .map(|c| atlas.vertex(c.into()))
                        .collect::<Vec<Vertex>>()
                })
                .collect::<Vec<Vec<Vertex>>>();
            world
                .create_entity()
                .with(KeyboardControlled)
                .with(MouseControlled)
                .with(Geometry(geometries))
                .with(Velocity { x: 0, y: 0 })
                .build();
        }
        Value::GeometryCollection(collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            let geometries: Vec<Geometry> = collection.into_par_iter().collect();
            for geom in geometries {
                match_geometry(geom, world, atlas);
            }
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

/// Process top-level GeoJSON items
fn process_geojson(gj: GeoJson, world: &mut World, atlas: &Atlas) {
    match gj {
        GeoJson::FeatureCollection(collection) => {
            let geometries: Vec<Geometry> = collection
                .features
                // Iterate in parallel where appropriate
                .into_par_iter()
                // Only pass on non-empty geometries
                .filter_map(|feature| feature.geometry)
                .collect();
            for geom in geometries {
                match_geometry(geom, world, atlas);
            }
        }
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry {
                match_geometry(geometry, world, atlas)
            }
        }
        GeoJson::Geometry(geometry) => match_geometry(geometry, world, atlas),
    }
}

trait IntoGeometryList {
    fn into_geometry_list(&self, atlas: &Atlas) -> Vec<Vec<Vertex>>;
}
impl IntoGeometryList for GeoJson {
    fn into_geometry_list(&self, atlas: &Atlas) -> Vec<Vec<Vertex>> {
        quick_collection(&self)
            .unwrap()
            .iter()
            .map(|geometry| {
                geometry
                    .coords_iter()
                    .map(|c| atlas.vertex(c.into()))
                    .collect::<Vec<Vertex>>()
            })
            .collect()
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let atlas = Atlas::new(screen);
    let window = video_subsys
        .window("Atlas", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(mouse::Mouse, "Mouse", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "Mouse"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);

    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    let mut mouse_command: MouseCommand = MouseCommand::None;
    world.insert(mouse_command.clone());
    world.insert(movement_command);
    world.insert(Camera::new());

    let contents = load_str!("./../assets/custom.geojson");
    let geojson = contents.parse::<GeoJson>().unwrap();

    process_geojson(geojson, &mut world, &atlas);

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        let mut movement_command = None;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                Event::MouseButtonDown { x, y, .. } => mouse_command = MouseCommand::Click((x, y)),
                Event::MouseMotion { x, y, .. } => {
                    mouse_command = mouse_command.update(Some((x, y)))
                }
                Event::MouseButtonUp { x, y, .. } => {
                    mouse_command = MouseCommand::Release((x, y));
                }

                _ => {}
            }
        }
        *world.write_resource() = movement_command;
        *world.write_resource() = mouse_command;
        println!("mouse command: {:?}", mouse_command);
        // Update
        dispatcher.dispatch(&mut world);
        world.maintain();
        mouse_command = mouse_command.update(None);
        // Render
        renderer::render(world.system_data(), &mut canvas)?;

        // Time management!
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 260));
    }

    Ok(())
}
