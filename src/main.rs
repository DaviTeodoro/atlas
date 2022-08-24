mod components;
mod physics;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

const PLAYER_MOVEMENT_SPEED: i32 = 6;

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// create animation frames for the standard character spritesheet
fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (width, height) = (top_left_frame.width(), top_left_frame.height());
    let y_offset = direction_spritesheet_row(direction) * height as i32 + top_left_frame.y();

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + i * width as i32,
                y_offset,
                width,
                height,
            ),
        });
    }
    frames
}

fn update_player(player: &mut Player) {
    match player.direction {
        Direction::Up => player.position = player.position.offset(0, -player.speed),
        Direction::Down => player.position = player.position.offset(0, player.speed),
        Direction::Left => player.position = player.position.offset(-player.speed, 0),
        Direction::Right => player.position = player.position.offset(player.speed, 0),
    }
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();

    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width as u32, frame_height as u32);

    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // It has to stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Atlas", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video video_subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let textures = [texture_creator.load_texture("assets/bardo.png")?];

    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };

    let mut world = World::new();

    world
        .create_entity()
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    //events
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = 0;
                }

                _ => {}
            }
        }

        // update
        i = (i + 1) % 360;
        update_player(&mut player);

        // render
        // render(&mut canvas, &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
    Ok(())
}
