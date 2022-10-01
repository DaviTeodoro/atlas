extern crate sdl2;
use geo::CoordsIter;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use triangulate::builders;
use triangulate::Triangulate;

use geojson::{quick_collection, GeoJson};

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
impl Into<Point> for geo_types::Coordinate {
    fn into(self) -> Point {
        Point::new(self.y, self.x)
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

trait GeoTriangulate {
    fn triangulatex(&self) -> Option<Vec<Point>>;
}
impl GeoTriangulate for geo_types::Geometry {
    fn triangulatex(&self) -> Option<Vec<Point>> {
        match self {
            geo_types::Geometry::Point { .. } => None,
            geo_types::Geometry::Line { .. } => None,
            geo_types::Geometry::LineString { .. } => None,
            geo_types::Geometry::Polygon { .. } => {
                let coords = self.coords_iter().map(|c| c.into()).collect::<Vec<Point>>();
                println!("coords: {:?}", coords);
                match triangulate(vec![coords]) {
                    Some(triangles) => {
                        println!("triangles: {:?}", triangles);
                        Some(triangles)
                    }
                    None => {
                        println!("triangles: None");
                        None
                    }
                }
            }
            geo_types::Geometry::MultiPoint { .. } => None,
            geo_types::Geometry::MultiLineString { .. } => None,
            geo_types::Geometry::MultiPolygon { .. } => None,
            geo_types::Geometry::GeometryCollection { .. } => None,
            geo_types::Geometry::Rect { .. } => None,
            geo_types::Geometry::Triangle { .. } => None,
        }
    }
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
                    }
                    canvas.present();
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;

                    let geojson_str = r#"
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {
        "stroke-width": 2,
        "stroke-opacity": 1,
        "fill-opacity": 0.5,
        "name": "Brasil"
      },
      "geometry": {
        "type": "Polygon",
        "coordinates": [
          [
            [
              -75.23,
              -34.59
            ],
            [
              -33.75,
              -34.59
            ],
            [
              -33.75,
              4.21
            ],
            [
              -75.23,
              4.21
            ],
            [
              -75.23,
              -34.59
            ]
          ]
        ]
      }
    }
  ]
}
"#;
                    let geojson = geojson_str.parse::<GeoJson>().unwrap();
                    // Turn the GeoJSON string into a geo_types GeometryCollection
                    quick_collection(&geojson)
                        .unwrap()
                        .iter()
                        .for_each(|geometry| match geometry.triangulatex() {
                            Some(points) => {
                                for i in 0..(points.iter().count() / 3) {
                                    let point_a = points[(i * 3)];
                                    let point_b = points[(i * 3) + 1];
                                    let point_c = points[(i * 3) + 2];

                                    let trigon: Vec<Vertex> = vec![
                                        atlas.vertex(point_a),
                                        atlas.vertex(point_b),
                                        atlas.vertex(point_c),
                                    ];
                                    println!("trigon {:?}", trigon);

                                    canvas
                                        .filled_trigon(
                                            trigon[0].x,
                                            trigon[0].y,
                                            trigon[2].x,
                                            trigon[2].y,
                                            trigon[1].x,
                                            trigon[1].y,
                                            pixels::Color::RGB(50 * i as u8, 51, 234),
                                        )
                                        .expect("failed to draw triangle");
                                }
                            }
                            None => {
                                println!("failed to triangulate")
                            }
                        });
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
    let a_polygons: Vec<Point> = vec![
        (-34.59704151614416, -75.234375).into(),
        (-34.59704151614416, -33.75).into(),
        (4.214943141390651, -33.75).into(),
        (4.214943141390651, -75.234375).into(),
        // (-34.59704151614416, -75.234375).into(),
    ];
    let t_polygons: Vec<Point> = vec![
        (-54.140625, 9.795677582829743).into(),
        (-16.875, -1.7575368113083125).into(),
        (-6.328125, -29.22889003019423).into(),
        (-18.6328125, -51.835777520452474).into(),
        (-49.21875, -60.06484046010449).into(),
        (-62.22656249999999, -55.37911044801048).into(), // (-34.59704151614416, -75.234375).into(),
    ];
    let b_polygons: Vec<Point> = vec![
        (-40., -1.).into(),
        (-44., -32.).into(),
        (-10., -38.).into(),
        (-10., -13.).into(),
        (-12., 1.).into(),
        (-17., 7.).into(),
        // (-40.078125, -1.4061088354351594).into(),
    ];
    let result = t_polygons
        .triangulate::<builders::FanToListAdapter<_, builders::VecListBuilder<_>>>(&mut Vec::new())
        .ok()?
        .iter()
        .map(|point| point.clone())
        .collect();
    Some(result)
}
