use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use geo_types::{MultiPolygon, Polygon};
use geojson::{GeoJson, Geometry, Value};

use geo::CoordsIter;

use rayon::prelude::*;

use crate::{vertex::Vertex, Point};

#[derive(Component)]
struct CameraBBox;

#[derive(Component)]
struct Velocity;

struct GreetTimer(Timer);

fn greet_features(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    // query: Query<&Geometry, With<Geometry>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello !");
    }
}

fn match_geometry(geom: Geometry, window_size: Vec2, commands: &mut Commands) {
    match geom.value {
        Value::Polygon(_) => {
            let poly: Polygon<f64> = geom.value.try_into().expect("Unable to convert Polygon");
            let polygon = poly.coords_iter().map(|c| c.into()).collect::<Vec<Point>>();
            let (screen_width, screen_height) = (window_size.x, window_size.y);

            let mut path_builder = PathBuilder::new();

            for (i, point) in polygon.iter().enumerate() {
                let vertex = point.get_screen_space_pos(screen_height);
                if i == 0 {
                    path_builder.move_to(Vec2::new(vertex.x, vertex.y));
                } else {
                    path_builder.line_to(Vec2::new(vertex.x, vertex.y));
                }
            }

            let line = path_builder.build();

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &line,
                    // DrawMode::Fill(FillMode::color(Color::CYAN)),
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::CYAN),
                        outline_mode: StrokeMode::new(Color::BLACK, 0.05),
                    },
                    Transform::default(),
                ))
                .insert(Velocity);
        }
        Value::MultiPolygon(_) => {
            let poly: MultiPolygon<f64> = geom
                .value
                .try_into()
                .expect("Unable to convert MultiPolygon");
            let polygons = poly
                .into_iter()
                .map(|p| p.coords_iter().map(|c| c.into()).collect::<Vec<Point>>())
                .collect::<Vec<Vec<Point>>>();

            for polygon in polygons {
                let screen_height = window_size.y;

                let mut path_builder = PathBuilder::new();

                for (i, point) in polygon.iter().enumerate() {
                    let vertex = point.get_screen_space_pos(screen_height);
                    if i == 0 {
                        path_builder.move_to(Vec2::new(vertex.x, vertex.y));
                    } else {
                        path_builder.line_to(Vec2::new(vertex.x, vertex.y));
                    }
                }

                let line = path_builder.build();

                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &line,
                        // DrawMode::Fill(FillMode::color(Color::CYAN)),
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::CYAN),
                            outline_mode: StrokeMode::new(Color::BLACK, 0.05),
                        },
                        Transform::default(),
                    ))
                    .insert(Velocity);
            }
        }
        Value::GeometryCollection(collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            let geometries: Vec<Geometry> = collection.into_par_iter().collect();
            for geom in geometries {
                match_geometry(geom, window_size, commands);
            }
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

fn process_geojson(gj: GeoJson, window_size: Vec2, commands: &mut Commands) {
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
                match_geometry(geom, window_size, commands);
            }
        }
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry {
                match_geometry(geometry, window_size, commands)
            }
        }
        GeoJson::Geometry(geometry) => match_geometry(geometry, window_size, commands),
    }
}

fn fetch_features(
    mut commands: Commands,
    windows: Res<Windows>,

    mut query: Query<(&mut Transform, &OrthographicProjection)>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE);

    for (camera_pos, cam_proj) in &mut query {
        if let Some(mouse_normalized_screen_pos) = mouse_normalized_screen_pos {
            let proj_size = Vec2::new(cam_proj.right, cam_proj.top);
            let mouse_world_pos = camera_pos.translation.truncate()
                + mouse_normalized_screen_pos * cam_proj.scale * proj_size;

            let point = Vertex {
                x: mouse_world_pos.x,
                y: mouse_world_pos.y,
            }
            .into_point(window_size);
            println!("cursor_pos {:?}", point);
        }
        let point = Vertex {
            x: camera_pos.translation.x,
            y: camera_pos.translation.y,
        }
        .into_point(window_size);
        println!("camera_pos {:?}", point)
    }
}

fn setup_camera_bbox(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.));
    path_builder.line_to(Vec2::new(0., 100.));
    let line = path_builder.build();

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            Transform::default(),
        ))
        .insert(CameraBBox);
}

fn update_camera_bbox(
    mut query_bbox: Query<&mut Transform, With<CameraBBox>>,
    mut query_camera_transform: Query<(&Camera, &mut Transform), Without<CameraBBox>>,
) {
    for (_, camera_transform) in &mut query_camera_transform.iter_mut() {
        for mut bbox_transform in query_bbox.iter_mut() {
            bbox_transform.translation = camera_transform.translation.clone();
        }
    }
}

fn load_features(mut commands: Commands, windows: Res<Windows>) {
    let contents = load_str!("./../assets/custom.geojson");
    let geojson = contents.parse::<GeoJson>().unwrap();
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);
    path_builder.line_to(Vec2::new(100., 100.));
    let line = path_builder.build();
    // commands
    //     .spawn_bundle(GeometryBuilder::build_as(
    //         &line,
    //         DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
    //         Transform::default(),
    //     ))
    //     .insert(Velocity);
    process_geojson(geojson, window_size, &mut commands);
}
pub struct LoadFeatures;

impl Plugin for LoadFeatures {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup_camera_bbox)
            .add_startup_system(load_features)
            .add_system(update_camera_bbox)
            .add_system(fetch_features)
            .add_system(greet_features);
    }
}
