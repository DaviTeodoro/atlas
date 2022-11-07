use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
// Camera
#[derive(Component)]
pub struct Camera;

fn zoom_to(
    mut query: Query<(&Camera, &mut OrthographicProjection, &mut Transform)>,
    mut scroll_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
) {
    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .iter()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll == 0. {
        return;
    }

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let mouse_normalized_screen_pos = window
        .cursor_position()
        .map(|cursor_pos| (cursor_pos / window_size) * 2. - Vec2::ONE);

    for (cam, mut proj, mut pos) in &mut query {
        let old_scale = proj.scale;
        proj.scale = (proj.scale * (1. + -scroll * 0.003)).max(0.01).min(2.);

        let proj_size = Vec2::new(proj.right, proj.top);
        if let Some(mouse_normalized_screen_pos) = mouse_normalized_screen_pos {
            let mouse_world_pos =
                pos.translation.truncate() + mouse_normalized_screen_pos * proj_size * old_scale;
            pos.translation = (mouse_world_pos
                - mouse_normalized_screen_pos * proj_size * proj.scale)
                .extend(pos.translation.z);
        }
    }
}

fn move_to(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<(&Camera, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let current_pos = match window.cursor_position() {
        Some(current_pos) => current_pos,
        None => return,
    };

    for (cam, mut transform, projection) in &mut query {
        let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

        if vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle]
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn))
        {
            let proj_size = Vec2::new(
                projection.right - projection.left,
                projection.top - projection.bottom,
            ) * projection.scale;
            let world_units_per_device_pixel = proj_size / window_size;

            let delta_world = delta_device_pixels * world_units_per_device_pixel;
            let mut proposed_cam_transform = transform.translation - delta_world.extend(0.);

            transform.translation = proposed_cam_transform;
        }
    }
    *last_pos = Some(current_pos)
}

fn load_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Camera);
}

pub struct LoadCamera;

impl Plugin for LoadCamera {
    fn build(&self, app: &mut App) {
        app.add_system(move_to)
            .add_system(zoom_to)
            .add_startup_system(load_camera);
    }
}
