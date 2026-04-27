use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

const MOVING_SPEED: f32 = 450.0;
const MOVE_FIELD_SIZE: f32 = 100.;

pub fn camera_movement(
    camera_query: Single<(&mut Transform, &Projection), With<Camera2d>>,
    time: Res<Time<Fixed>>,
    window: Single<&Window>,
) {
    let (mut transform, proj) = camera_query.into_inner();

    let Projection::Orthographic(proj2d) = proj else {
        return;
    };

    let fspeed = MOVING_SPEED * proj2d.scale * time.delta_secs();

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    if cursor.y < MOVE_FIELD_SIZE {
        transform.translation.y += fspeed;
    }
    if window.height() - cursor.y < MOVE_FIELD_SIZE {
        transform.translation.y -= fspeed;
    }
    if cursor.x < MOVE_FIELD_SIZE {
        transform.translation.x -= fspeed;
    }
    if window.width() - cursor.x < MOVE_FIELD_SIZE {
        transform.translation.x += fspeed;
    }
}

pub fn camera_zoom(
    mut camera: Query<(&Camera, &mut Transform, &GlobalTransform, &mut Projection)>,
    window: Single<&Window>,
    mut wheel: MessageReader<MouseWheel>,
) {
    if wheel.read().len() == 0 {
        return;
    }

    let scale = {
        wheel.read().fold(1., |scale, ev| {
            let units = match ev.unit {
                MouseScrollUnit::Line => ev.y,
                MouseScrollUnit::Pixel => ev.y / 20.0,
            };

            scale * ops::powf(0.85, units)
        })
    };

    let Ok((camera, mut transform, global, mut projection)) = camera.single_mut() else {
        return;
    };

    let Projection::Orthographic(proj2d) = &mut *projection else {
        return;
    };

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let Ok(world_before) = camera.viewport_to_world_2d(global, cursor) else {
        return;
    };

    proj2d.scale *= scale;

    let cam_xy = transform.translation.truncate();
    let new_cam = world_before * (1.0 - scale) + cam_xy * scale;

    transform.translation.x = new_cam.x;
    transform.translation.y = new_cam.y;
}
