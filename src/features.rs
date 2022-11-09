use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use geo_types::{MultiPolygon, Polygon};
use geojson::{GeoJson, Geometry, Value};

use geo::CoordsIter;

use rayon::prelude::*;

use crate::{vertex::Vertex, Point};

#[derive(Component)]
struct CameraBBox;

struct GreetTimer(Timer);

fn greet_features(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    // query: Query<&Geometry, With<Geometry>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // println!("hello !");
    }
}

fn log_cursor_pos(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &OrthographicProjection)>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE);

    for (camera_pos, cam_proj) in &mut query {
        let proj_size = Vec2::new(cam_proj.right, cam_proj.top);
        if let Some(mouse_normalized_screen_pos) = mouse_normalized_screen_pos {
            let mouse_world_pos = camera_pos.translation.truncate()
                + mouse_normalized_screen_pos * cam_proj.scale * proj_size;

            let point = Vertex {
                x: mouse_world_pos.x,
                y: mouse_world_pos.y,
            }
            .into_point(window_size);
            println!("cursor_pos {:?}", point);
        }
    }
}

fn setup_camera_bbox(mut commands: Commands) {
    // println!("window_size {:?}", window_size);
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.));
    let line = path_builder.build();
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            transform,
        ))
        .insert(CameraBBox);
}

fn update_camera_bbox(
    mut query_bbox: Query<(&mut Path, &mut DrawMode), With<CameraBBox>>,
    mut query_camera_transform: Query<
        (&mut Transform, &OrthographicProjection),
        Without<CameraBBox>,
    >,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    for (camera_transform, cam_proj) in &mut query_camera_transform.iter_mut() {
        // println!("camera to bbox {:?}", camera.get_bbox());
        for (mut bbox_path, mut bbox_draw_mode) in &mut query_bbox.iter_mut() {
            let camera_bbox = get_camera_bbox(cam_proj, *camera_transform, window_size);
            let camera_bbox_polygon: Polygon<f64> =
                get_camera_bbox(cam_proj, *camera_transform, window_size).to_polygon();
            // println!(
            //     "tiles in view: {:?}",
            //     camera_bbox.get_tile_list(cam_proj.scale)
            // );
            *bbox_draw_mode =
                DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0 * cam_proj.scale));
            *bbox_path = camera_bbox_polygon.into_path(window.height());
        }
    }
}

fn get_camera_bbox(
    cam_proj: &OrthographicProjection,
    camera_pos: Transform,
    window_size: Vec2,
) -> geo_types::Rect<f64> {
    fn normalized_into_world_pos(
        pos: Vec2,
        cam_proj: &OrthographicProjection,
        cam_pos: &Transform,
    ) -> Vertex {
        let proj_size = Vec2::new(cam_proj.right, cam_proj.top);
        let word_pos = cam_pos.translation.truncate() + pos * cam_proj.scale * proj_size;
        Vertex {
            x: word_pos.x,
            y: word_pos.y,
        }
    }

    let top_left_corner = normalized_into_world_pos(Vec2::new(-1.0, 1.0), cam_proj, &camera_pos)
        .into_point(window_size);

    let bottom_right_corner =
        normalized_into_world_pos(Vec2::new(1.0, -1.0), cam_proj, &camera_pos)
            .into_point(window_size);

    geo_types::Rect::new(
        geo_types::Coordinate {
            x: top_left_corner.lng as f64,
            y: top_left_corner.lat as f64,
        },
        geo_types::Coordinate {
            x: bottom_right_corner.lng as f64,
            y: bottom_right_corner.lat as f64,
        },
    )
}

fn load_features(mut commands: Commands, windows: Res<Windows>) {
    let contents = load_str!("./../assets/custom.geojson");
    let buffer = load_bytes!("./../assets/0.pbf");
    println!("pbf {:?}", buffer);
    let geojson = contents.parse::<GeoJson>().unwrap();
    let window = windows.get_primary().unwrap();
    // https://maps.ckochis.com/data/v3/1/1/1.pbf
    process_geojson(geojson, window.height(), &mut commands);
}

trait RectExt {
    fn get_tile_list(&self, zoom: f32) -> Vec<(i32, i32, i32)>;
}

impl RectExt for geo_types::Rect {
    fn get_tile_list(&self, cam_scale: f32) -> Vec<(i32, i32, i32)> {
        let zoom = (1.0 / cam_scale.sqrt()) as i32;
        let coords: Vec<Point> = self.coords_iter().map(|coord| coord.into()).collect();
        let top_left_corner = coords[1];
        let bottom_right_corner = coords[3];

        let min_tile = top_left_corner.get_tile(zoom);
        let max_tile = bottom_right_corner.get_tile(zoom);

        let (min_x, max_x) = (min_tile.0, max_tile.0);
        let (min_y, max_y) = (min_tile.1, max_tile.1);

        let mut tiles = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                tiles.push((x, y, zoom));
            }
        }
        tiles
    }
}

trait PolygonExt {
    fn into_path(&self, screen_height: f32) -> bevy_prototype_lyon::entity::Path;
}

impl PolygonExt for Polygon<f64> {
    fn into_path(&self, screen_height: f32) -> bevy_prototype_lyon::entity::Path {
        let polygon: Vec<Point> = self.coords_iter().map(|coord| coord.into()).collect();
        let mut path_builder = PathBuilder::new();
        let mut first = true;
        for point in polygon.iter() {
            let vertex = point.get_screen_space_pos(screen_height);
            if first {
                path_builder.move_to(Vec2::new(vertex.x, vertex.y));
                first = false;
            } else {
                path_builder.line_to(Vec2::new(vertex.x, vertex.y));
            }
        }
        path_builder.build()
    }
}

fn match_geometry(geom: Geometry, screen_height: f32, commands: &mut Commands) {
    match geom.value {
        Value::Polygon(_) => {
            let polygon: Polygon<f64> = geom.value.try_into().expect("Unable to convert Polygon");

            let line = polygon.into_path(screen_height);
            commands.spawn_bundle(GeometryBuilder::build_as(
                &line,
                // DrawMode::Fill(FillMode::color(Color::CYAN)),
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.05),
                },
                Transform::default(),
            ));
        }
        Value::MultiPolygon(_) => {
            let multi_polygon: MultiPolygon<f64> = geom
                .value
                .try_into()
                .expect("Unable to convert MultiPolygon");
            let polygons: Vec<Polygon<f64>> = multi_polygon.into_iter().collect();

            for polygon in polygons {
                let line = polygon.into_path(screen_height);

                commands.spawn_bundle(GeometryBuilder::build_as(
                    &line,
                    // DrawMode::Fill(FillMode::color(Color::CYAN)),
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::CYAN),
                        outline_mode: StrokeMode::new(Color::BLACK, 0.05),
                    },
                    Transform::default(),
                ));
            }
        }
        Value::GeometryCollection(collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            let geometries: Vec<Geometry> = collection.into_par_iter().collect();
            for geom in geometries {
                match_geometry(geom, screen_height, commands);
            }
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

fn process_geojson(gj: GeoJson, screen_height: f32, commands: &mut Commands) {
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
                match_geometry(geom, screen_height, commands);
            }
        }
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry {
                match_geometry(geometry, screen_height, commands)
            }
        }
        GeoJson::Geometry(geometry) => match_geometry(geometry, screen_height, commands),
    }
}

pub struct LoadFeatures;

impl Plugin for LoadFeatures {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup_camera_bbox)
            .add_startup_system(load_features)
            .add_system(update_camera_bbox)
            // .add_system(log_cursor_pos)
            .add_system(greet_features);
    }
}
