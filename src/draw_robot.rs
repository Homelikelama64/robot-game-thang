use raylib::prelude::*;

use crate::{Map, Robot, Rotation};

pub fn draw_robot(
    d: &mut RaylibDrawHandle,
    robot: &Robot,
    map_width: f32,
    map: &Map,
    max_brain_area_width: f32,
    buffer: f32,
    width: i32,
    height: i32,
    robot_asset: &Texture2D,
) {
    let scale = map_width / map.width as f32;

    d.draw_rectangle_v(
        Vector2 {
            x: max_brain_area_width + buffer,
            y: buffer,
        },
        Vector2 {
            x: map.width as f32 * scale,
            y: map.height as f32 * scale,
        },
        Color::new(70, 70, 70, 255),
    );

    d.draw_line_ex(
        Vector2 {
            x: max_brain_area_width + map_width + buffer * 2.0,
            y: 0.0,
        },
        Vector2 {
            x: max_brain_area_width + map_width + buffer * 2.0,
            y: height as f32,
        },
        width as f32 / 250.0,
        Color::new(255, 255, 255, 255),
    );
    let rotation = match robot.rotation {
        Rotation::Up => 0.0,
        Rotation::Right => 90.0,
        Rotation::Down => 180.0,
        Rotation::Left => 270.0,
    };
    d.draw_texture_pro(
        robot_asset,
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: robot_asset.width as f32,
            height: robot_asset.height as f32,
        },
        Rectangle {
            x: max_brain_area_width + buffer + scale / 2.0 + robot.pos.0 as f32 * scale,
            y: (scale * map.height as f32) - robot.pos.1 as f32 * scale + buffer + scale / 2.0
                - scale,
            width: scale,
            height: scale,
        },
        Vector2 {
            x: scale / 2.0,
            y: scale / 2.0,
        },
        rotation,
        Color::new(255, 255, 255, 255),
    );
}
