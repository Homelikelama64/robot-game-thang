use raylib::prelude::*;
use rodio::buffer;

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
    let (buffer_x, buffer_y) = draw_ui_boarders(
        d,
        size,
        scale,
        assets,
        bottom_left_pos,
        size / brain.width as f32 * brain.height as f32,
    );
    let width = size - buffer_x * 2.0;
    let instruction_size = width / brain.width as f32;
    let height = instruction_size * brain.height as f32;
    let top_left_pos = Vector2::new(
        bottom_left_pos.x + buffer_x,
        bottom_left_pos.y - buffer_y - height,
    );
    //Brain
    for (i, instruction) in brain.instructions.iter().enumerate() {
        let grid_pos = Vector2::new(
            (i % brain.width as usize) as f32,
            (i / brain.width as usize) as f32,
        );
        let pos = Vector2::new(
            (grid_pos.x * instruction_size) + top_left_pos.x,
            (-grid_pos.y - 1.0) * instruction_size + top_left_pos.y + height,
        );
        draw_instruction(
            d,
            instruction,
            instruction_size,
            assets,
            pos,
            instruction.instruction_type == InstructionType::None,
        );
        if grid_pos.x as i32 == brain.reader.pos.0 && grid_pos.y as i32 == brain.reader.pos.1 {
            let offset = Vector2::new(instruction_size / 2.0, instruction_size / 2.0);
            d.draw_texture_pro(
                &assets.reader,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: assets.reader.width as f32,
                    height: assets.reader.height as f32,
                },
                Rectangle {
                    x: pos.x + offset.x,
                    y: pos.y + offset.y,
                    width: instruction_size,
                    height: instruction_size,
                },
                offset,
                0.0,
                Color::WHITE,
            );
        }
    }
    let mouse_pos_brain = Vector2::new(
        ((mouse_pos.x - top_left_pos.x) / instruction_size).floor(),
        (-(mouse_pos.y - top_left_pos.y - height) / instruction_size).floor(),
    );
    let mouse_pos_rounded = Vector2::new(
        (mouse_pos_brain.x * instruction_size) + top_left_pos.x,
        (-mouse_pos_brain.y - 1.0) * instruction_size + top_left_pos.y + height,
    );
    let mouse_index =
        mouse_pos_brain.x as usize + mouse_pos_brain.y as usize * brain.width as usize;
    if brain.in_bounds((mouse_pos_brain.x as i32, mouse_pos_brain.y as i32)) {
        draw_instruction(
            d,
            selected_instruction,
            instruction_size,
            assets,
            mouse_pos_rounded,
            selected_instruction.instruction_type == InstructionType::None
                || (*selected_instruction != brain.instructions[mouse_index]
                    && (selected_instruction.instruction_type != InstructionType::Move
                        || selected_instruction.instruction_type != InstructionType::Back
                        || selected_instruction.instruction_type != InstructionType::RotateLeft
                        || selected_instruction.instruction_type != InstructionType::RotateRight)),
        );
    }
    let avalible_instructions = brain.get_avalible_instructions();
    let selection_grid_width = 5;
    let instruction_size = width / selection_grid_width as f32;
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y + buffer_y * 2.0);
    let selection_grid_height =
        (avalible_instructions.len() as f32 / selection_grid_width as f32).ceil() as i32;
    let height = selection_grid_height as f32 * instruction_size;
    draw_ui_boarders(
        d,
        size,
        scale,
        assets,
        top_left_pos
            + Vector2 {
                x: 0.0,
                y: height + buffer_y,
            },
        height + buffer_y * 2.0,
    );
    let top_left_pos = Vector2::new(top_left_pos.x + buffer_x, top_left_pos.y);
    for i in 0..(selection_grid_width * selection_grid_height) as usize {
        let pos = Vector2::new(
            top_left_pos.x + (i % selection_grid_width as usize) as f32 * instruction_size,
            top_left_pos.y + (i / selection_grid_width as usize) as f32 * instruction_size,
        );

        let instruction = if i < avalible_instructions.len() {
            if avalible_instructions[i].1 == selected_instruction.instruction_type {
                Instruction {
                    instruction_type: avalible_instructions[i].1,
                    rotation: selected_instruction.rotation,
                    edit: false,
                }
            } else {
                Instruction {
                    instruction_type: avalible_instructions[i].1,
                    rotation: Rotation::Up,
                    edit: false,
                }
            }
        } else {
            Instruction {
                instruction_type: InstructionType::Blank,
                rotation: Rotation::Up,
                edit: false,
            }
        };

        let up = selected_instruction.instruction_type != instruction.instruction_type
            || i >= avalible_instructions.len();
        let instruction_count = brain.get_instruction_count(instruction.instruction_type);
        draw_instruction(d, &instruction, instruction_size, assets, pos, up);
        if i < avalible_instructions.len() && instruction_count != 1 {
            d.draw_text(
                instruction_count.to_string().as_str(),
                (pos.x + instruction_size * 0.6) as i32,
                (pos.y + instruction_size * 0.75) as i32,
                (instruction_size * 0.25) as i32,
                Color::WHITE,
            );
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
    let mut rotation = match instruction.rotation {
        crate::Rotation::Up => 0.0,
        crate::Rotation::Right => 90.0,
        crate::Rotation::Down => 180.0,
        crate::Rotation::Left => 270.0,
    };
    let texture = match instruction.instruction_type {
        crate::InstructionType::Move => {
            rotation = 0.0;
            &assets.move_instruction
        }
        crate::InstructionType::Back => {
            rotation = 0.0;
            &assets.back_instruction
        }
        crate::InstructionType::Direction => &assets.direction_instruction,
        crate::InstructionType::RotateLeft => {
            rotation = 0.0;
            &assets.left_instruction
        }
        crate::InstructionType::RotateRight => {
            rotation = 0.0;
            &assets.right_instruction
        }
        crate::InstructionType::None => &assets.blank_instruction,
        InstructionType::Blank => &assets.blank_instruction,
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
    scale: f32,
    assets: &Assets,
    bottom_left_pos: Vector2,
    height: f32,
) -> (f32, f32) {
    let top_left_pos = Vector2::new(bottom_left_pos.x, bottom_left_pos.y - height);
    {
        let width = size - (assets.brain_corner.width as f32 * 2.0) * scale;
        let height = height - (assets.brain_corner.width as f32) * scale;
        let boarder_height = assets.brain_edge.height as f32;
        let offset = Vector2::new(width / 2.0, boarder_height / 2.0 * scale);
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
                x: bottom_left_pos.x + assets.brain_corner.width as f32 * scale + width / 2.0,
                y: bottom_left_pos.y - boarder_height / 2.0 * scale,
                width,
                height: boarder_height * scale,
            },
            offset,
            0.0,
            Color::WHITE,
        );
        //Top
        d.draw_texture_pro(
            &assets.brain_edge,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: assets.brain_edge.width as f32,
                height: assets.brain_edge.height as f32,
            },
            Rectangle {
                x: bottom_left_pos.x + offset.y * 2.0 + width / 2.0,
                y: bottom_left_pos.y - boarder_height / 2.0 * scale - height,
                width,
                height: boarder_height * scale,
            },
            offset,
            180.0,
            Color::WHITE,
        );
        let offset = Vector2::new(
            (height - offset.y * 2.0) / 2.0,
            boarder_height / 2.0 * scale,
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
                x: bottom_left_pos.x + offset.y,
                y: bottom_left_pos.y - offset.y - height / 2.0,
                width: height - offset.y * 2.0,
                height: boarder_height * scale,
            },
            offset,
            90.0,
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
                x: bottom_left_pos.x + offset.y * 3.0 + width,
                y: bottom_left_pos.y - offset.y - height / 2.0,
                width: height - offset.y * 2.0,
                height: boarder_height * scale,
            },
            offset,
            -90.0,
            Color::WHITE,
        );
    }
    {
        let width = assets.brain_corner.width as f32 * scale;
        let height = assets.brain_corner.height as f32 * scale;
        //let origin = Vector2::new(
        //    -assets.brain_corner.width as f32 / 2.0,
        //    assets.brain_corner.width as f32 / 2.0,
        //);
        let origin = Vector2::new(width / 2.0, height / 2.0);
        let offset = Vector2::new(origin.x, origin.y);
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
                x: bottom_left_pos.x + offset.x,
                y: bottom_left_pos.y - offset.y,
                width,
                height,
            },
            origin,
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
                x: bottom_left_pos.x - offset.x + size,
                y: bottom_left_pos.y - offset.y,
                width,
                height,
            },
            origin,
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
                x: top_left_pos.x + offset.x,
                y: top_left_pos.y + offset.y,
                width,
                height,
            },
            origin,
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
                x: top_left_pos.x - offset.x + size,
                y: top_left_pos.y + offset.y,
                width,
                height,
            },
            origin,
            180.0,
            Color::WHITE,
        );
    }
    //d.draw_rectangle_rec(
    //    Rectangle {
    //        x: bottom_left_pos.x,
    //        y: bottom_left_pos.y - height,
    //        width: size,
    //        height,
    //    },
    //    Color::new(255, 255, 255, 10),
    //);
    //d.draw_rectangle_rec(
    //    Rectangle {
    //        x: bottom_left_pos.x + assets.brain_corner.width as f32 * scale,
    //        y: bottom_left_pos.y - height + assets.brain_corner.height as f32 * scale,
    //        width: size - assets.brain_corner.width as f32 * scale * 2.0,
    //        height: height - assets.brain_corner.height as f32 * scale * 2.0,
    //    },
    //    Color::new(255, 255, 255, 10),
    //);
    (
        assets.brain_corner.width as f32 * scale,
        assets.brain_corner.height as f32 * scale,
    )
}
