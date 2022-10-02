extern crate sdl2;
use geo::CoordsIter;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;

use geojson::{quick_collection, GeoJson};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let atlas = Atlas::new(screen);
    let mut camera = Camera::new();
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

    let mut is_draging = false;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(226, 232, 240));
    canvas.clear();

    let geojson = geojson_str().parse::<GeoJson>().unwrap();
    draw(&geojson, &mut canvas, &atlas, &camera);

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
                    // let color = pixels::Color::RGB(x as u8, y as u8, 255);

                    println!("mouse btn down at (x:{},y:{})", x, y);
                    is_draging = true;
                    canvas.present();
                }

                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    ..
                } => {
                    println!("mouse motion at (x:{},y:{})", x, y);
                    println!("isDraging {}", is_draging);
                    if is_draging {
                        camera.move_to(x as u32, y as u32)
                    };
                    canvas.clear();
                    draw(&geojson, &mut canvas, &atlas, &camera);
                }

                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    is_draging = false;
                    println!("mouse btn up at (x:{},y:{})", x, y);
                }

                _ => {}
            }
        }
    }

    Ok(())
}

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

#[derive(Debug)]
struct Camera {
    x: u32,
    y: u32,
    prev_x: u32,
    prev_y: u32,
    delta_x: i32,
    delta_y: i32,
    zoom: f64,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
            prev_x: SCREEN_WIDTH / 2,
            prev_y: SCREEN_HEIGHT / 2,
            delta_x: 0,
            delta_y: 0,
            zoom: 1.0,
        }
    }
    fn move_to(&mut self, x: u32, y: u32) {
        self.prev_x = self.x;
        self.prev_y = self.y;
        self.x = x;
        self.y = y;
        self.delta_x = self.x as i32 - self.prev_x as i32;
        self.delta_y = self.y as i32 - self.prev_y as i32;
    }
}

fn draw(
    geojson: &GeoJson,
    canvas: &mut Canvas<sdl2::video::Window>,
    atlas: &Atlas,
    camera: &Camera,
) {
    quick_collection(&geojson)
        .unwrap()
        .iter()
        .for_each(|geometry| {
            let coords = geometry
                .coords_iter()
                .map(|c| c.into())
                .collect::<Vec<Point>>();
            let vertices = coords
                .iter()
                .map(|c| atlas.vertex(*c))
                .collect::<Vec<Vertex>>();
            // println!("coords: {:?}", vertices);
            println!("camera: {:?}", camera);
            let (vx, vy) = vertices.iter().fold((vec![], vec![]), |acc, vertex| {
                (
                    [acc.0, vec![vertex.x + camera.delta_x as i16]].concat(),
                    [acc.1, vec![vertex.y + camera.delta_y as i16]].concat(),
                )
            });
            canvas
                .filled_polygon(&vx, &vy, pixels::Color::RGB(171, 191, 218))
                .expect("failed to draw triangle");
        });
    canvas.present();
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

fn geojson_str() -> String {
    r#"
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
              -52.20703125,
              -32.842673631954305
            ],
            [
              -50.44921875,
              -30.44867367928756
            ],
            [
              -48.515625,
              -28.30438068296277
            ],
            [
              -48.603515625,
              -25.997549919572098
            ],
            [
              -47.109375,
              -24.5271348225978
            ],
            [
              -44.8681640625,
              -23.443088931121775
            ],
            [
              -43.9892578125,
              -22.958393318086337
            ],
            [
              -42.84667968749999,
              -23.1605633090483
            ],
            [
              -41.87988281249999,
              -22.593726063929296
            ],
            [
              -41.37451171875,
              -22.177231792821342
            ],
            [
              -40.95703125,
              -21.983801417384697
            ],
            [
              -41.055908203125,
              -21.442843107187652
            ],
            [
              -40.528564453125,
              -20.725290873994197
            ],
            [
              -40.220947265625,
              -20.293113447544098
            ],
            [
              -39.891357421875,
              -19.642587534013032
            ],
            [
              -39.7265625,
              -19.207428526801184
            ],
            [
              -39.715576171875,
              -18.63583516062284
            ],
            [
              -39.627685546875,
              -18.145851771694467
            ],
            [
              -39.144287109375,
              -17.675427818339383
            ],
            [
              -39.210205078125,
              -17.23525150539052
            ],
            [
              -38.97949218749999,
              -16.14081555527601
            ],
            [
              -38.902587890625,
              -15.802824941413187
            ],
            [
              -39.00146484375,
              -15.082731671605787
            ],
            [
              -39.056396484375,
              -14.732386081418454
            ],
            [
              -38.97949218749999,
              -14.237762492417659
            ],
            [
              -38.9520263671875,
              -13.2399454992863
            ],
            [
              -38.199462890625,
              -12.88677980084704
            ],
            [
              -37.913818359375,
              -12.46876014482322
            ],
            [
              -37.452392578125,
              -11.662996112308035
            ],
            [
              -37.276611328125,
              -11.221510260010541
            ],
            [
              -36.6943359375,
              -10.617418067950293
            ],
            [
              -36.37573242187499,
              -10.395974612177643
            ],
            [
              -36.23291015625,
              -10.266276060027122
            ],
            [
              -35.8428955078125,
              -9.768611091236483
            ],
            [
              -35.7110595703125,
              -9.67656858750112
            ],
            [
              -35.65063476562499,
              -9.606166114941969
            ],
            [
              -35.4913330078125,
              -9.373192635083441
            ],
            [
              -35.3375244140625,
              -9.215982405510825
            ],
            [
              -35.22216796875,
              -9.03157787963176
            ],
            [
              -35.1507568359375,
              -8.912206892260215
            ],
            [
              -35.1177978515625,
              -8.781939521035971
            ],
            [
              -35.068359375,
              -8.68420891954859
            ],
            [
              -34.9859619140625,
              -8.477805461808186
            ],
            [
              -34.9639892578125,
              -8.363692651835823
            ],
            [
              -34.903564453125,
              -8.211490323420682
            ],
            [
              -34.8651123046875,
              -8.02659484248955
            ],
            [
              -34.82666015625,
              -7.852498637813016
            ],
            [
              -34.8211669921875,
              -7.732765062729807
            ],
            [
              -34.83215332031249,
              -7.5694373362514344
            ],
            [
              -34.8101806640625,
              -7.280741242677959
            ],
            [
              -34.771728515625,
              -7.160850096497242
            ],
            [
              -34.8101806640625,
              -7.002763681982734
            ],
            [
              -34.892578125,
              -6.882800241767556
            ],
            [
              -34.93377685546875,
              -6.798262304540396
            ],
            [
              -34.9639892578125,
              -6.678247482748998
            ],
            [
              -34.99420166015625,
              -6.585488564561947
            ],
            [
              -34.98870849609375,
              -6.356245636365608
            ],
            [
              -35.07659912109375,
              -6.208820977124995
            ],
            [
              -35.1177978515625,
              -5.971217057997224
            ],
            [
              -35.16448974609375,
              -5.883796361755705
            ],
            [
              -35.2001953125,
              -5.692515936746771
            ],
            [
              -35.24139404296874,
              -5.512107462769789
            ],
            [
              -35.36224365234374,
              -5.337113527125014
            ],
            [
              -35.5517578125,
              -5.123772299948803
            ],
            [
              -35.92529296875,
              -5.069057826784033
            ],
            [
              -36.397705078125,
              -5.112829778499449
            ],
            [
              -36.683349609375,
              -5.145656780300514
            ],
            [
              -37.177734375,
              -4.937724274302479
            ],
            [
              -37.50732421875,
              -4.455950571647079
            ],
            [
              -38.16650390625,
              -4.171115454867424
            ],
            [
              -38.47412109375,
              -3.688855143147035
            ],
            [
              -38.97949218749999,
              -3.35988909487339
            ],
            [
              -39.6826171875,
              -3.008869978848142
            ],
            [
              -40.31982421875,
              -2.8772079526533365
            ],
            [
              -41.28662109375,
              -2.943040910055132
            ],
            [
              -41.748046875,
              -2.986927393334863
            ],
            [
              -43.33007812499999,
              -2.3723687086440504
            ],
            [
              -44.560546875,
              -2.064982495867104
            ],
            [
              -46.142578125,
              -1.098565496040652
            ],
            [
              -47.4609375,
              -0.7909904981540058
            ],
            [
              -48.603515625,
              -0.4833927027896987
            ],
            [
              -50.71289062499999,
              -0.08789059053082422
            ],
            [
              -50.1416015625,
              1.3182430568620136
            ],
            [
              -50.8447265625,
              3.0746950723696944
            ],
            [
              -51.67968749999999,
              4.214943141390651
            ],
            [
              -52.734375,
              2.3723687086440504
            ],
            [
              -55.283203125,
              2.28455066023697
            ],
            [
              -56.33789062499999,
              1.7575368113083254
            ],
            [
              -57.919921875,
              1.6696855009865839
            ],
            [
              -59.150390625,
              1.4939713066293239
            ],
            [
              -60.380859375,
              2.8991526985043135
            ],
            [
              -59.58984374999999,
              4.390228926463396
            ],
            [
              -60.380859375,
              5.266007882805498
            ],
            [
              -62.22656249999999,
              3.9519408561575946
            ],
            [
              -64.86328125,
              3.8642546157214084
            ],
            [
              -62.9296875,
              1.7575368113083254
            ],
            [
              -65.7421875,
              0.7031073524364909
            ],
            [
              -67.1484375,
              1.5818302639606454
            ],
            [
              -70.13671875,
              1.2303741774326145
            ],
            [
              -70.13671875,
              -0.4394488164139641
            ],
            [
              -69.60937499999999,
              -1.7575368113083125
            ],
            [
              -69.60937499999999,
              -4.127285323245357
            ],
            [
              -71.71875,
              -4.302591077119676
            ],
            [
              -73.037109375,
              -5.7908968128719565
            ],
            [
              -73.740234375,
              -7.18810087117902
            ],
            [
              -73.388671875,
              -9.188870084473393
            ],
            [
              -71.3671875,
              -9.88227549342994
            ],
            [
              -70.224609375,
              -10.40137755454354
            ],
            [
              -66.357421875,
              -10.40137755454354
            ],
            [
              -64.072265625,
              -12.554563528593656
            ],
            [
              -62.13867187499999,
              -13.2399454992863
            ],
            [
              -60.732421875,
              -14.179186142354169
            ],
            [
              -59.853515625,
              -16.13026201203474
            ],
            [
              -58.00781249999999,
              -17.308687886770024
            ],
            [
              -57.65624999999999,
              -18.979025953255267
            ],
            [
              -58.27148437499999,
              -20.055931265194438
            ],
            [
              -57.74414062500001,
              -22.350075806124853
            ],
            [
              -56.07421875,
              -22.512556954051437
            ],
            [
              -55.107421875,
              -23.88583769986199
            ],
            [
              -54.052734375,
              -24.607069137709694
            ],
            [
              -53.525390625,
              -26.11598592533351
            ],
            [
              -53.96484375,
              -27.761329874505233
            ],
            [
              -56.51367187499999,
              -29.458731185355315
            ],
            [
              -58.00781249999999,
              -30.29701788337204
            ],
            [
              -56.953125,
              -30.44867367928756
            ],
            [
              -56.33789062499999,
              -30.67571540416773
            ],
            [
              -54.4921875,
              -31.353636941500987
            ],
            [
              -52.20703125,
              -32.842673631954305
            ]
          ]
        ]
      }
    }
  ]
}
"#
    .to_owned()
}
