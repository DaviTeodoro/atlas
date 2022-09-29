extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;
use triangulate::builders;
use triangulate::{Triangulate, Vertex};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
struct VertexTest {
    x: i16,
    y: i16,
}

impl Into<VertexTest> for (f32, f32) {
    fn into(self) -> VertexTest {
        VertexTest {
            x: self.0 as i16,
            y: self.1 as i16,
        }
    }
}

impl VertexTest {
    fn new(x: i16, y: i16) -> Self {
        VertexTest { x, y }
    }
}
impl Vertex for VertexTest {
    type Coordinate = f32;

    #[inline(always)]
    fn x(&self) -> Self::Coordinate {
        self.x as f32
    }

    #[inline(always)]
    fn y(&self) -> Self::Coordinate {
        self.y as f32
    }
}
//
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct VTest {
    x: f32,
    y: f32,
}

impl VTest {
    pub fn new(x: f32, y: f32) -> Self {
        VTest { x, y }
    }
}

impl std::fmt::Debug for VTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for VTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vertex for VTest {
    type Coordinate = f32;

    #[inline(always)]
    fn x(&self) -> Self::Coordinate {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> Self::Coordinate {
        self.y
    }
}

impl Into<VTest> for (f32, f32) {
    fn into(self) -> VTest {
        VTest::new(self.0, self.1)
    }
}
//

fn square() -> Vec<VertexTest> {
    vec![
        (0.0, 0.0).into(),
        (100.0, 0.0).into(),
        (100.0, 100.0).into(),
        (0.0, 100.0).into(),
    ]
}

struct Screen {
    width: u32,
    height: u32,
}

#[derive(Default, Copy, Clone)]
struct Point {
    lat: f64,
    lng: f64,
}

struct Atlas {
    screen: Screen,
}

impl Atlas {
    fn new(screen: Screen) -> Atlas {
        Atlas { screen }
    }
    //TODO: return vec<trigon> for feature
    // fn draw(&self, geometry: Vec<usize>) -> Vec<Vec<(f64, f64)>> {
    //     let screen = &self.screen;
    //     let (x, y) = screen.pixels_to_percent(100, 100);
    //     let point = screen.percent_to_pixels(x, y);
    //     let polygons: Vec<Vec<(f64, f64)>> = vec![
    //         vec![(0., 0.), (0., 1.), (1., 1.), (1., 0.)],
    //         vec![(0.05, 0.05), (0.05, 0.95), (0.95, 0.95), (0.95, 0.05)],
    //     ];
    //     // let output: Vec<Vec<(f64,f64)>> = polygons.triangulate_default::<builders::VecVecFanBuilder<_>>()?;
    //     // polygons
    //     //     .triangulate_default::<builders::VecVecIndexedFanBuilder<_>>()
    //     //     .expect("Triangulation failed");
    // }

    fn vertex(&self, point: Point) -> VertexTest {
        let screen = &self.screen;
        let (x, y) = from_lng_lat(point);
        screen.percent_to_pixels(x, y)
    }
}

impl Screen {
    fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }
    fn percent_to_pixels(&self, percent_x: f64, percent_y: f64) -> VertexTest {
        let x = self.width;
        let y = self.height;

        VertexTest {
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
                        let point_a = VertexTest { x: 400, y: 99 };
                        let point_b = VertexTest { x: 611, y: 424 };
                        let point_c = VertexTest { x: 196, y: 430 };
                        for i in 0..10 {
                            canvas.filled_trigon(
                                point_a.x + i,
                                point_a.y,
                                point_b.x + i,
                                point_b.y,
                                point_c.x + i,
                                point_c.y,
                                pixels::Color::RGB(147, 51, 234),
                            )?;
                        }
                        canvas.present();
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

                    let polygons: Vec<Vec<VTest>> = vec![
                        vec![
                            (0., 0.).into(),
                            (0., 1.).into(),
                            (1., 1.).into(),
                            (1., 0.).into(),
                        ],
                        vec![
                            (0.05, 0.05).into(),
                            (0.05, 0.95).into(),
                            (0.95, 0.95).into(),
                            (0.95, 0.05).into(),
                        ],
                    ];

                    match polygons.triangulate::<builders::VecVecFanBuilder<_>>(&mut Vec::new()) {
                        Ok(triangulation) => {
                            for fan in triangulation {
                                print!("(");
                                for vertex in fan {
                                    print!("({:?}, {:?}), ", vertex.x, vertex.y);
                                }
                                print!("), ");
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }

                    println!("mouse btn down at (x:{},y:{})", x, y);
                    println!("from_lng_lat (x:{:?})", from_lng_lat(new_york));
                    // println!(
                    //     "percent_to_pixels ({:?})",
                    //     screen.percent_to_pixels(0.50, 0.50)
                    // );
                    // println!(
                    //     "pixels_to_percent(x:{:?})",
                    //     screen.pixels_to_percent(400, 300)
                    // );
                    println!("a vertex from a point (x:{:?})", atlas.vertex(new_york));
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
    //As TS: (180 - (180 / Math.PI * Math.log(Math.tan(Math.PI / 4 + lat * Math.PI / 360)))) / 360
    (180.0
        - (180.0 / std::f64::consts::PI
            * ((std::f64::consts::PI / 4.0 + lat * std::f64::consts::PI / 360.0).tan()).ln()))
        / 360.0
}

fn from_lng_lat(point: Point) -> (f64, f64) {
    (
        mercator_x_from_lng(point.lng),
        mercator_y_from_lat(point.lat),
    )
}
