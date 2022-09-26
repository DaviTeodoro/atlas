extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

struct Screen {
    width: u32,
    height: u32,
}

impl Screen {
    fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }
    fn percent_to_pixels(&self, percent_x: f64, percent_y: f64) -> Point {
        let x = self.width;
        let y = self.height;

        Point {
            x: (x as f64 * percent_x) as u32,
            y: (y as f64 * percent_y) as u32,
        }
    }
    fn pixels_to_percent(&self, x: u32, y: u32) -> (f64, f64) {
        let width = self.width;
        let height = self.height;

        ((x as f64 / width as f64), (y as f64 / height as f64))
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            screen.width,
            screen.height,
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
                        for i in 0..1501 {
                            canvas.filled_trigon(
                                400,
                                99,
                                611,
                                424,
                                196,
                                430,
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
                    println!("mouse btn down at (x:{},y:{})", x, y);
                    println!("from_lng_lat (x:{:?})", from_lng_lat(-73.9911, 40.7343));
                    println!(
                        "percent_to_pixels ({:?})",
                        screen.percent_to_pixels(
                            from_lng_lat(-73.9911, 40.7343).0,
                            from_lng_lat(-73.9911, 40.7343).1
                        )
                    );
                    println!(
                        "percent_to_pixels ({:?})",
                        screen.percent_to_pixels(0.50, 0.50)
                    );
                    println!(
                        "percent_to_pixels(x:{:?})",
                        screen.pixels_to_percent(400, 300)
                    );
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

fn from_lng_lat(lng: f64, lat: f64) -> (f64, f64) {
    (mercator_x_from_lng(lng), mercator_y_from_lat(lat))
}
