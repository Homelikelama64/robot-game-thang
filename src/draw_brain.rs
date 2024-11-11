use raylib::prelude::*;

use crate::{Assets, Brain, Instruction, InstructionType, Rotation};

pub fn draw_brain(
    d: &mut RaylibDrawHandle,
    brain: &Brain,
    bottom_left_pos: Vector2,
    size: f32,
    assets: &Assets,
    mouse_pos: Vector2,
    selected_instruction: &Instruction,
    scale: f32,
) {
    let size = size * scale;
    let width = size - assets.brain_corner.width as f32 * 2.0;
    let instruction_size = width / brain.width as f32;
    let height = brain.height as f32 * instruction_size + assets.brain_corner.height as f32 * 2.0;

    draw_ui_boarders(d, size, assets, bottom_left_pos, height);
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height)
        + Vector2::new(
            assets.brain_corner.width as f32,
            assets.brain_corner.height as f32,
        );

    let mouse_brain_pos = Vector2::new(
        ((mouse_pos.x - top_left_pos.x) / instruction_size).floor(),
        (-(mouse_pos.y - top_left_pos.y - brain.height as f32 * instruction_size)
            / instruction_size)
            .floor(),
    );
    for instruction in 0..brain.instructions.len() {
        let pos = Vector2::new(
            top_left_pos.x + (instruction as i32 % brain.width as i32) as f32 * (instruction_size),
            bottom_left_pos.y
                - instruction_size
                - assets.brain_corner.height as f32
                - ((instruction as i32 / brain.width as i32) as f32 * (instruction_size)),
        );
        let instruction = &brain.instructions[instruction];

        draw_instruction(
            d,
            instruction,
            instruction_size,
            assets,
            pos,
            instruction.instruction_type == InstructionType::None,
        );
    }
    {
        let pos = Vector2::new(
            mouse_brain_pos.x * instruction_size + top_left_pos.x,
            -mouse_brain_pos.y * instruction_size + top_left_pos.y + height
                - instruction_size
                - assets.brain_corner.height as f32 * 2.0,
        );
        let instruction =
            brain.get_instruction((mouse_brain_pos.x as i32, mouse_brain_pos.y as i32));
        if brain.in_bounds((mouse_brain_pos.x as i32, mouse_brain_pos.y as i32)) && instruction.edit
        {
            draw_instruction(
                d,
                selected_instruction,
                instruction_size,
                assets,
                pos,
                selected_instruction != instruction
                    || selected_instruction.instruction_type == InstructionType::None,
            );
        }
    }
    {
        let avalible_instructions = brain.get_avalible_instructions();
        let options_width = 5;
        let options_height =
            (avalible_instructions.len() as f32 / options_width as f32).ceil() as i32;
        let instruction_size =
            (size - assets.brain_corner.width as f32 * 2.0) / options_width as f32;

        let height =
            options_height as f32 * instruction_size + assets.brain_corner.height() as f32 * 2.0;

        let bottom_left_pos = bottom_left_pos
            + Vector2 {
                x: 0.0,
                y: 16.0 + options_height as f32 * height,
            };
        let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);

        draw_ui_boarders(d, size, assets, bottom_left_pos, height);

        for i in 0..options_width * options_height {
            let pos = Vector2::new(
                top_left_pos.x
                    + assets.brain_corner.width as f32
                    + (i as f32 % options_width as f32).floor() * instruction_size,
                top_left_pos.y
                    + assets.brain_corner.height as f32
                    + (i as f32 / options_width as f32).floor() * instruction_size,
            );
            let mut up = false;
            let instruction = if i >= avalible_instructions.len() as i32 {
                &Instruction {
                    instruction_type: InstructionType::None,
                    rotation: Rotation::Up,
                    edit: false,
                }
            } else {
                up = selected_instruction.instruction_type == avalible_instructions[i as usize].1;
                if avalible_instructions[i as usize].1 == selected_instruction.instruction_type {
                    &Instruction {
                        instruction_type: avalible_instructions[i as usize].1,
                        rotation: selected_instruction.rotation,
                        edit: false,
                    }
                } else {
                    &Instruction {
                        instruction_type: avalible_instructions[i as usize].1,
                        rotation: Rotation::Up,
                        edit: false,
                    }
                }
            };

            draw_instruction(d, instruction, instruction_size, assets, pos, up);

            if i < avalible_instructions.len() as i32 && avalible_instructions[i as usize].0 != 1 {
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

fn draw_instruction(
    d: &mut RaylibDrawHandle,
    instruction: &Instruction,
    instruction_size: f32,
    assets: &Assets,
    pos: Vector2,
    up: bool,
) {
    let texture = match instruction.instruction_type {
        crate::InstructionType::Move => &assets.move_instruction,
        crate::InstructionType::Direction => &assets.direction_instruction,
        crate::InstructionType::RotateLeft => todo!(),
        crate::InstructionType::RotateRight => todo!(),
        crate::InstructionType::None => &assets.blank_instruction,
    };
    let rotation = match instruction.rotation {
        crate::Rotation::Up => 0.0,
        crate::Rotation::Right => 90.0,
        crate::Rotation::Down => 180.0,
        crate::Rotation::Left => 270.0,
    };
    let offset = Vector2::new(instruction_size / 2.0, instruction_size / 2.0);
    let boarder_texture = match up {
        true => &assets.up_instruction,
        false => &assets.down_instruction,
    };
    d.draw_texture_pro(
        boarder_texture,
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: boarder_texture.width as f32,
            height: boarder_texture.height as f32,
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

fn draw_ui_boarders(
    d: &mut RaylibDrawHandle,
    size: f32,
    assets: &Assets,
    bottom_left_pos: Vector2,
    height: f32,
) {
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);
    {
        let width = size - assets.brain_corner.width as f32 * 2.0;
        let boarder_height = assets.brain_edge.height as f32;
        //Bottom
        d.draw_texture_pro(
            &assets.brain_edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_edge.width as f32,
                height: assets.brain_edge.height as f32,
            },
            Rectangle {
                x: bottom_left_pos.x + assets.brain_corner.width as f32 + width / 2.0,
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
            &assets.brain_edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_edge.width as f32,
                height: assets.brain_edge.height as f32,
            },
            Rectangle {
                x: (bottom_left_pos.x + boarder_height / 2.0),
                y: bottom_left_pos.y
                    - (height - assets.brain_corner.height as f32 * 2.0) / 2.0
                    - assets.brain_corner.height as f32,
                width: height - assets.brain_corner.height as f32 * 2.0,
                height: boarder_height,
            },
            Vector2::new(
                (height - assets.brain_corner.height as f32 * 2.0) / 2.0,
                boarder_height / 2.0,
            ),
            90.0,
            Color::WHITE,
        );
        //Up
        d.draw_texture_pro(
            &assets.brain_edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_edge.width as f32,
                height: assets.brain_edge.height as f32,
            },
            Rectangle {
                x: top_left_pos.x + assets.brain_corner.width as f32 + width / 2.0,
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
            &assets.brain_edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_edge.width as f32,
                height: assets.brain_edge.height as f32,
            },
            Rectangle {
                x: (bottom_left_pos.x - boarder_height / 2.0 + size),
                y: (bottom_left_pos.y - width / 2.0 - assets.brain_corner.height as f32),
                width: height - assets.brain_corner.height as f32 * 2.0,
                height: boarder_height,
            },
            Vector2::new(width / 2.0, boarder_height / 2.0),
            -90.0,
            Color::WHITE,
        );
    }
    {
        let width = assets.brain_corner.width as f32;
        let height = assets.brain_corner.height as f32;
        //Bottom Left
        d.draw_texture_pro(
            &assets.brain_corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_corner.width as f32,
                height: assets.brain_corner.height as f32,
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
            &assets.brain_corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_corner.width as f32,
                height: assets.brain_corner.height as f32,
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
            &assets.brain_corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_corner.width as f32,
                height: assets.brain_corner.height as f32,
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
            &assets.brain_corner,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_corner.width as f32,
                height: assets.brain_corner.height as f32,
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
