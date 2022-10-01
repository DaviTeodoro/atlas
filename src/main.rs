extern crate sdl2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use triangulate::builders;
use triangulate::Triangulate;

use geojson::{Feature, GeoJson, Geometry, Value};
use std::convert::TryFrom;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
struct Vertex {
    x: i16,
    y: i16,
}

impl Into<Vertex> for (f32, f32) {
    fn into(self) -> Vertex {
        Vertex {
            x: self.0 as i16,
            y: self.1 as i16,
        }
    }
}

//
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    lat: f64,
    lng: f64,
}

impl Point {
    pub fn new(lat: f64, lng: f64) -> Self {
        Point { lat, lng }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lng)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lng)
    }
}

impl triangulate::Vertex for Point {
    type Coordinate = f32;

    #[inline(always)]
    fn x(&self) -> Self::Coordinate {
        self.lat as f32
    }

    #[inline(always)]
    fn y(&self) -> Self::Coordinate {
        self.lng as f32
    }
}

impl Into<Point> for (f64, f64) {
    fn into(self) -> Point {
        Point::new(self.1, self.0)
    }
}
//

struct Screen {
    width: u32,
    height: u32,
}

struct Atlas {
    screen: Screen,
}

impl Atlas {
    fn new(screen: Screen) -> Atlas {
        Atlas { screen }
    }

    fn vertex(&self, point: Point) -> Vertex {
        let screen = &self.screen;
        let (x, y) = from_lng_lat(point);
        screen.percent_to_pixels(x, y)
    }
}

impl Screen {
    fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }
    fn percent_to_pixels(&self, percent_x: f64, percent_y: f64) -> Vertex {
        let x = self.width;
        let y = self.height;

        Vertex {
            x: (x as f64 * percent_x) as i16,
            y: (y as f64 * percent_y) as i16,
        }
    }
    // fn pixels_to_percent(&self, x: u32, y: u32) -> (f64, f64) {
    //     let width = self.width;
    //     let height = self.height;

    //     ((x as f64 / width as f64), (y as f64 / height as f64))
    // }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let atlas = Atlas::new(screen);
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(226, 232, 240));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");

                        let geojson_str = r#"
                                            {
                                              "type": "Feature",
                                              "properties": { "food": "donuts" },
                                              "geometry": {
                                                "type": "Point",
                                                "coordinates": [ -118.2836, 34.0956 ]
                                              }
                                            }
                                            "#;
                        let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
                        let feature: Feature = Feature::try_from(geojson).unwrap();
                        println!("feature: {:?}", feature);
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;

                    let new_york = Point {
                        lat: -73.9911,
                        lng: 40.7386,
                    };

                    let usa_bound_box: Vec<Vec<Point>> = vec![vec![
                        (-126., 23.).into(),
                        (-60., 23.).into(),
                        (-60., 50.).into(),
                        (-126., 50.).into(),
                    ]];

                    match triangulate(usa_bound_box) {
                        Some(points) => {
                            for i in 0..(points.iter().count() & 3) {
                                let point_a = points[(i * 3)];
                                let point_b = points[(i * 3) + 1];
                                let point_c = points[(i * 3) + 2];

                                let trigon: Vec<Vertex> = vec![
                                    atlas.vertex(point_a),
                                    atlas.vertex(point_b),
                                    atlas.vertex(point_c),
                                ];

                                canvas.filled_trigon(
                                    trigon[0].x,
                                    trigon[0].y,
                                    trigon[2].x,
                                    trigon[2].y,
                                    trigon[1].x,
                                    trigon[1].y,
                                    pixels::Color::RGB(50 * i as u8, 51, 234),
                                )?;
                            }
                        }
                        None => {}
                    }

                    println!("mouse btn down at (x:{},y:{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}

fn mercator_x_from_lng(lng: f64) -> f64 {
    (lng + 180.0) / 360.0
}

fn mercator_y_from_lat(lat: f64) -> f64 {
    let sin_lat = lat.to_radians().sin();
    0.5 - ((1.0 + sin_lat) / (1.0 - sin_lat)).log(std::f64::consts::E)
        / (4.0 * std::f64::consts::PI)
}

fn from_lng_lat(point: Point) -> (f64, f64) {
    (
        mercator_x_from_lng(point.lng),
        mercator_y_from_lat(point.lat),
    )
}

fn triangulate(polygons: Vec<Vec<Point>>) -> Option<Vec<Point>> {
    let result = polygons
        .triangulate::<builders::VecVecFanBuilder<_>>(&mut Vec::new())
        .ok()?
        .iter()
        .map(|fan| {
            if fan.len() == 3 {
                Some(vec![
                    (fan[0].lng, fan[0].lat).into(),
                    (fan[1].lng, fan[1].lat).into(),
                    (fan[2].lng, fan[2].lat).into(),
                ])
            } else if fan.len() == 4 {
                Some(vec![
                    (fan[0].lng, fan[0].lat).into(),
                    (fan[1].lng, fan[1].lat).into(),
                    (fan[2].lng, fan[2].lat).into(),
                    (fan[0].lng, fan[0].lat).into(),
                    (fan[2].lng, fan[2].lat).into(),
                    (fan[3].lng, fan[3].lat).into(),
                ])
            } else {
                None
            }
        })
        .collect::<Option<Vec<Vec<_>>>>()?
        .into_iter()
        .flat_map(|x| x.into_iter())
        .collect();
    Some(result)
}
