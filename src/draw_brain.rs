use raylib::prelude::*;

use crate::{Asset, Brain, Instruction};

pub fn draw_brain(
    d: &mut RaylibDrawHandle,
    brain: &Brain,
    bottom_left_pos: Vector2,
    size: f32,
    edge: &Texture2D,
    corner: &Texture2D,
    blank_instruction_asset: &Asset,
    move_instruction_asset: &Asset,
    direction_instruction_asset: &Asset,
    mouse_pos: Vector2,
    selected_instruction: &Instruction,
    scale: f32,
) {
    let size = size * scale;
    let width = size - corner.width as f32 * 2.0;
    let instruction_size = width / brain.width as f32;
    let height = brain.height as f32 * instruction_size + corner.height as f32 * 2.0;

    draw_ui_boarders(d, size, edge, corner, bottom_left_pos, height);
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height)
        + Vector2::new(corner.width as f32, corner.height as f32);
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
            crate::InstructionType::Move => &move_instruction_asset.down,
            crate::InstructionType::Direction => &direction_instruction_asset.down,
            crate::InstructionType::RotateLeft => todo!(),
            crate::InstructionType::RotateRight => todo!(),
            crate::InstructionType::None => {
                rotation = 0.0;
                &blank_instruction_asset.down
            }
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
    {
        let avalible_instructions = brain.get_avalible_instructions();
        let options_width = 5;
        let options_height =
            (avalible_instructions.len() as f32 / options_width as f32).ceil() as i32;
        let instruction_size = (size - corner.width as f32 * 2.0) / options_width as f32;

        let height = options_height as f32 * instruction_size + corner.height() as f32 * 2.0;

        let bottom_left_pos = bottom_left_pos
            + Vector2 {
                x: 0.0,
                y: 16.0 + options_height as f32 * height,
            };
        let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);

        draw_ui_boarders(d, size, edge, corner, bottom_left_pos, height);

        for i in 0..options_width * options_height {
            let mut instruction_texture = if i >= avalible_instructions.len() as i32 {
                &blank_instruction_asset.down
            } else {
                match avalible_instructions[i as usize].1 {
                    crate::InstructionType::Move => &move_instruction_asset.down,
                    crate::InstructionType::Direction => &direction_instruction_asset.down,
                    crate::InstructionType::RotateLeft => todo!(),
                    crate::InstructionType::RotateRight => todo!(),
                    crate::InstructionType::None => &blank_instruction_asset.down,
                }
            };
            let pos = Vector2::new(
                top_left_pos.x
                    + corner.width as f32
                    + (i as f32 % options_width as f32).floor() * instruction_size,
                top_left_pos.y
                    + corner.height as f32
                    + (i as f32 / options_width as f32).floor() * instruction_size,
            );
            if i < avalible_instructions.len() as i32 {
                if matches!(avalible_instructions[i as usize].1, ()) {
                    instruction_texture = match avalible_instructions[i as usize].1 {
                        crate::InstructionType::Move => &move_instruction_asset.up,
                        crate::InstructionType::Direction => &direction_instruction_asset.up,
                        crate::InstructionType::RotateLeft => todo!(),
                        crate::InstructionType::RotateRight => todo!(),
                        crate::InstructionType::None => &blank_instruction_asset.up,
                    };
                }
            }

            d.draw_texture_pro(
                instruction_texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: instruction_texture.width as f32,
                    height: instruction_texture.height as f32,
                },
                Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: instruction_size,
                    height: instruction_size,
                },
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
            if i < avalible_instructions.len() as i32 {
                d.draw_text(
                    avalible_instructions[i as usize].0.to_string().as_str(),
                    (pos.x + instruction_size - instruction_size / 3.0) as i32,
                    (pos.y + instruction_size - instruction_size / 3.0) as i32,
                    (instruction_size / 3.0) as i32,
                    Color::WHITE,
                );
            }
        }
    }
}

fn draw_ui_boarders(
    d: &mut RaylibDrawHandle,
    size: f32,
    edge: &Texture2D,
    corner: &Texture2D,
    bottom_left_pos: Vector2,
    height: f32,
) {
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);
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
