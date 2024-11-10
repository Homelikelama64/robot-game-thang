use raylib::prelude::*;

use crate::Brain;

pub fn draw_brain(
    d: &mut RaylibDrawHandle,
    brain: &Brain,
    bottom_left_pos: Vector2,
    size: f32,
    edge: &Texture2D,
    corner: &Texture2D,
    blank_instruction_asset: &Texture2D,
    move_instruction_asset: &Texture2D,
    direction_instruction_asset: &Texture2D,
    scale: f32,
) {
    let size = size * scale;
    let width = size - corner.width as f32 * 2.0;
    let instruction_size = width / brain.width as f32;
    let height = brain.height as f32 * instruction_size + corner.height as f32 * 2.0;
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);

    draw_ui_boarders(d, size, edge, corner, bottom_left_pos, height, top_left_pos);
    let top_left_pos = top_left_pos + Vector2::new(corner.width as f32, corner.height as f32);
    for instruction in 0..brain.instructions.len() {
        let pos = Vector2::new(
            top_left_pos.x + (instruction as i32 % brain.width as i32) as f32 * (instruction_size),
            bottom_left_pos.y
                - instruction_size
                - corner.height as f32
                - ((instruction as i32 / brain.width as i32) as f32 * (instruction_size)),
        );
        let instruction = &brain.instructions[instruction];
        let offset = Vector2::new(instruction_size * 0.5, instruction_size * 0.5);

        let mut rotation = match instruction.rotation {
            crate::Rotation::Up => 0.0,
            crate::Rotation::Right => 90.0,
            crate::Rotation::Down => 180.0,
            crate::Rotation::Left => 270.0,
        };
        let texture = match instruction.instruction_type {
            crate::InstructionType::Move => move_instruction_asset,
            crate::InstructionType::Direction => direction_instruction_asset,
            crate::InstructionType::RotateLeft => todo!(),
            crate::InstructionType::RotateRight => todo!(),
            crate::InstructionType::None => {
                rotation = 0.0;
                blank_instruction_asset
            },
        };

        d.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: blank_instruction_asset.width as f32,
                height: blank_instruction_asset.height as f32,
            },
            Rectangle {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
                width: instruction_size,
                height: instruction_size,
            },
            offset,
            rotation,
            Color::WHITE,
        );
    }
}

fn draw_ui_boarders(
    d: &mut RaylibDrawHandle,
    size: f32,
    edge: &Texture2D,
    corner: &Texture2D,
    bottom_left_pos: Vector2,
    height: f32,
    top_left_pos: Vector2,
) {
    {
        let width = size - corner.width as f32 * 2.0;
        let boarder_height = edge.height as f32;
        //Bottom
        d.draw_texture_pro(
            edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: edge.width as f32,
                height: edge.height as f32,
            },
            Rectangle {
                x: bottom_left_pos.x + corner.width as f32 + width / 2.0,
                y: bottom_left_pos.y - boarder_height / 2.0,
                width,
                height: boarder_height,
            },
            Vector2::new(width / 2.0, boarder_height / 2.0),
            0.0,
            Color::WHITE,
        );
        //Left
        d.draw_texture_pro(
            edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: edge.width as f32,
                height: edge.height as f32,
            },
            Rectangle {
                x: (bottom_left_pos.x + boarder_height / 2.0),
                y: bottom_left_pos.y
                    - (height - corner.height as f32 * 2.0) / 2.0
                    - corner.height as f32,
                width: height - corner.height as f32 * 2.0,
                height: boarder_height,
            },
            Vector2::new(
                (height - corner.height as f32 * 2.0) / 2.0,
                boarder_height / 2.0,
            ),
            90.0,
            Color::WHITE,
        );
        //Up
        d.draw_texture_pro(
            edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: edge.width as f32,
                height: edge.height as f32,
            },
            Rectangle {
                x: top_left_pos.x + corner.width as f32 + width / 2.0,
                y: top_left_pos.y + boarder_height / 2.0,
                width,
                height: boarder_height,
            },
            Vector2::new(width / 2.0, boarder_height / 2.0),
            180.0,
            Color::WHITE,
        );
        //Right
        d.draw_texture_pro(
            edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: edge.width as f32,
                height: edge.height as f32,
            },
            Rectangle {
                x: (bottom_left_pos.x - boarder_height / 2.0 + size),
                y: (bottom_left_pos.y - width / 2.0 - corner.height as f32),
                width: height - corner.height as f32 * 2.0,
                height: boarder_height,
            },
            Vector2::new(width / 2.0, boarder_height / 2.0),
            -90.0,
            Color::WHITE,
        );
    }
    {
        let width = corner.width as f32;
        let height = corner.height as f32;
        //Bottom Left
        d.draw_texture_pro(
            corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: corner.width as f32,
                height: corner.height as f32,
            },
            Rectangle {
                x: bottom_left_pos.x + width / 2.0,
                y: bottom_left_pos.y - height / 2.0,
                width,
                height,
            },
            Vector2::new(width / 2.0, height / 2.0),
            0.0,
            Color::WHITE,
        );
        //Bottom Right
        d.draw_texture_pro(
            corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: corner.width as f32,
                height: corner.height as f32,
            },
            Rectangle {
                x: bottom_left_pos.x - width / 2.0 + size,
                y: bottom_left_pos.y - height / 2.0,
                width,
                height,
            },
            Vector2::new(width / 2.0, height / 2.0),
            -90.0,
            Color::WHITE,
        );
        //Top Left
        d.draw_texture_pro(
            corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: corner.width as f32,
                height: corner.height as f32,
            },
            Rectangle {
                x: top_left_pos.x + width / 2.0,
                y: top_left_pos.y + height / 2.0,
                width,
                height,
            },
            Vector2::new(width / 2.0, height / 2.0),
            90.0,
            Color::WHITE,
        );
        //Top Right
        d.draw_texture_pro(
            corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: corner.width as f32,
                height: corner.height as f32,
            },
            Rectangle {
                x: top_left_pos.x - width / 2.0 + size,
                y: top_left_pos.y + height / 2.0,
                width,
                height,
            },
            Vector2::new(width / 2.0, height / 2.0),
            180.0,
            Color::WHITE,
        );
    }
}
