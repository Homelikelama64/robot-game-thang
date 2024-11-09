use raylib::prelude::*;

use crate::{Brain, Instruction, InstructionType, Rotation};

pub fn draw_instructions(
    d: &mut RaylibDrawHandle,
    brain: &mut Brain,
    width: i32,
    height: i32,
    buffer: f32,
    max_brain_area_width: f32,
    y_off: f32,
    mouse_pos: Vector2,
    forwards_asset: &Texture2D,
    direction_asset: &Texture2D,
    blank_asset: &Texture2D,
    rotate_left_asset: &Texture2D,
    rotate_right_asset: &Texture2D,
    clear_asset: &Texture2D,
    reader_asset: &Texture2D,
    selected: Instruction,
) {
    // Instruction Drawing
    let scale = (max_brain_area_width - (buffer * 2.0)) / brain.width as f32;

    let brain_pixel_height = brain.height as f32 * scale;
    let brain_pixel_width = brain.width as f32 * scale;

    d.draw_rectangle_v(
        Vector2 {
            x: buffer,
            y: y_off,
        },
        Vector2 {
            x: (max_brain_area_width - (buffer * 2.0)),
            y: brain_pixel_height,
        },
        Color::new(70, 70, 70, 255),
    );
    let pos = Vector2 {
        x: brain.reader.pos.0 as f32 * scale + buffer + scale / 2.0,
        y: brain_pixel_height - (brain.reader.pos.1 as f32 * scale + scale / 2.0) + y_off,
    };

    let mouse_brain_pos = Vector2 {
        x: (mouse_pos.x - buffer) / scale,
        y: -(mouse_pos.y - brain_pixel_height - y_off) / scale,
    };

    for i in 0..brain.instructions.len() {
        let instruction = &brain.instructions[i];
        let brain_location = Vector2 {
            x: i as f32 % brain.width as f32,
            y: i as f32 / brain.width as f32,
        };
        let pos = Vector2::new(
            brain_location.x.floor() * scale + buffer + scale / 2.0,
            brain_pixel_height - brain_location.y.floor() * scale + y_off + scale / 2.0,
        );
        let texture = match instruction.instruction_type {
            InstructionType::Forwards => forwards_asset,
            InstructionType::Direction => direction_asset,
            InstructionType::None => blank_asset,
            InstructionType::RotateLeft => rotate_left_asset,
            InstructionType::RotateRight => rotate_right_asset,
        };
        let rotation = match instruction.rotation {
            Rotation::Up => 0.0,
            Rotation::Right => 90.0,
            Rotation::Down => 180.0,
            Rotation::Left => 270.0,
        };
        if !(mouse_brain_pos.x.floor() == brain_location.x.floor()
            && mouse_brain_pos.y.floor() == brain_location.y.floor())
        {
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: direction_asset.width as f32,
                    height: direction_asset.height as f32,
                },
                Rectangle {
                    x: pos.x,
                    y: pos.y - scale,
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
    }
    if brain.in_bounds((
        mouse_brain_pos.x.floor() as i32,
        mouse_brain_pos.y.floor() as i32,
    )) {
        let mouse_brain_pos = Vector2::new(
            mouse_brain_pos.x.floor() * scale + buffer + scale / 2.0,
            brain_pixel_height - mouse_brain_pos.y.floor() * scale + y_off + scale / 2.0,
        );
        let mut rotation = match selected.rotation {
            Rotation::Up => 0.0,
            Rotation::Right => 90.0,
            Rotation::Down => 180.0,
            Rotation::Left => 270.0,
        };
        let texture = match selected.instruction_type {
            InstructionType::Forwards => forwards_asset,
            InstructionType::Direction => direction_asset,
            InstructionType::None => {
                rotation = 0.0;
                clear_asset
            }
            InstructionType::RotateLeft => rotate_left_asset,
            InstructionType::RotateRight => rotate_right_asset,
        };
        d.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width as f32,
                height: texture.height as f32,
            },
            Rectangle {
                x: mouse_brain_pos.x,
                y: mouse_brain_pos.y - scale,
                width: scale,
                height: scale,
            },
            Vector2 {
                x: scale / 2.0,
                y: scale / 2.0,
            },
            rotation,
            Color::new(255, 255, 255, 200),
        );
    }
    d.draw_texture_pro(
        reader_asset,
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: reader_asset.width as f32,
            height: reader_asset.height as f32,
        },
        Rectangle {
            x: pos.x,
            y: pos.y,
            width: scale,
            height: scale,
        },
        Vector2 {
            x: scale / 2.0,
            y: scale / 2.0,
        },
        match brain.reader.rotation {
            Rotation::Up => 0.0,
            Rotation::Right => 90.0,
            Rotation::Down => 180.0,
            Rotation::Left => 270.0,
        },
        Color::new(255, 255, 255, 255),
    );

    for x in 0..=brain.width as i32 {
        let x = x as f32 * scale + buffer;
        d.draw_line_ex(
            Vector2 { x, y: y_off },
            Vector2 {
                x,
                y: y_off + brain_pixel_height,
            },
            scale / 10.0,
            Color::new(125, 125, 125, 255),
        );
    }
    for y in 0..=brain.height as i32 {
        let y = y as f32 * scale;
        d.draw_line_ex(
            Vector2 {
                x: buffer,
                y: y_off + y,
            },
            Vector2 {
                x: buffer + brain_pixel_width,
                y: y_off + y,
            },
            scale / 10.0,
            Color::new(125, 125, 125, 255),
        );
    }

    d.draw_line_ex(
        Vector2 {
            x: max_brain_area_width,
            y: 0.0,
        },
        Vector2 {
            x: max_brain_area_width,
            y: height as f32,
        },
        width as f32 / 250.0,
        Color::WHITE,
    );
}
