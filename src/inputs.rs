use std::{fs::File, io::BufReader};

use raylib::prelude::*;
use rodio::{Decoder, OutputStreamHandle, Source};

use crate::{Assets, BrainEdit, InstructionType, Map, Rotation, World};

pub fn inputs(
    rl: &mut RaylibHandle,
    world: &mut World,
    assets: &Assets,
    mouse_pos: Vector2,
    brain_edit: &mut BrainEdit,
    sound_handle: &OutputStreamHandle,
    stepping: bool,
) -> bool {
    let mut stepping = stepping;
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        stepping = !stepping;
    }
    if brain_edit.id.is_some() {
        brain(rl, world, assets, mouse_pos, sound_handle, brain_edit);
    }
    if rl.get_mouse_wheel_move() > 0.0 {
        brain_edit.selected_instruction.rotation = match brain_edit.selected_instruction.rotation {
            Rotation::Up => Rotation::Right,
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
        }
    } else if rl.get_mouse_wheel_move() < 0.0 {
        brain_edit.selected_instruction.rotation = match brain_edit.selected_instruction.rotation {
            Rotation::Up => Rotation::Left,
            Rotation::Right => Rotation::Up,
            Rotation::Down => Rotation::Right,
            Rotation::Left => Rotation::Down,
        }
    }

    stepping
}

fn brain(
    rl: &mut RaylibHandle,
    world: &mut World,
    assets: &Assets,
    mouse_pos: Vector2,
    sound_handle: &OutputStreamHandle,
    brain_edit: &mut BrainEdit,
) {
    let brain = &mut world.robots[brain_edit.id.unwrap()].brain;
    let buffer_x = assets.brain_corner.width as f32 * brain_edit.scale;
    let buffer_y = assets.brain_corner.height as f32 * brain_edit.scale;
    let width = brain_edit.size * brain_edit.scale - buffer_x * 2.0;
    let instruction_size = width / brain.width as f32;
    let mouse_brain_pos = Vector2::new(
        ((mouse_pos.x - (brain_edit.pos.x + buffer_x)) / instruction_size).floor(),
        (-((mouse_pos.y - (brain_edit.pos.y - buffer_y)) / instruction_size)).floor(),
    );
    let instruction_count =
        brain.get_instruction_count(brain_edit.selected_instruction.instruction_type);
    if brain.in_bounds((mouse_brain_pos.x as i32, mouse_brain_pos.y as i32)) {
        let index = mouse_brain_pos.x as usize + mouse_brain_pos.y as usize * brain.width as usize;
        let old_instruction = brain.instructions[index];
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            && (instruction_count != 0
                || brain_edit.selected_instruction.instruction_type
                    == old_instruction.instruction_type)
        {
            brain.instructions[index] = brain_edit.selected_instruction;
            if old_instruction != brain.instructions[index] {
                if brain_edit.selected_instruction.instruction_type != InstructionType::None {
                    let _ = sound_handle.play_raw(
                        Decoder::new(BufReader::new(
                            File::open("Assets/button_down.wav").unwrap(),
                        ))
                        .unwrap()
                        .convert_samples(),
                    );
                } else if old_instruction.instruction_type != InstructionType::None {
                    let _ = sound_handle.play_raw(
                        Decoder::new(BufReader::new(File::open("Assets/button_up.wav").unwrap()))
                            .unwrap()
                            .convert_samples(),
                    );
                }
            }
        }
    }

    let avalible_instructions = brain.get_avalible_instructions();
    let options_width = 5;
    let options_height = (avalible_instructions.len() as f32 / options_width as f32).ceil() as i32;
    let instruction_size =
        (brain_edit.size * brain_edit.scale - buffer_x * 2.0) / options_width as f32;
    let mouse_selection_pos = Vector2::new(
        ((mouse_pos.x - (brain_edit.pos.x + buffer_x)) / instruction_size).floor(),
        ((mouse_pos.y - (brain_edit.pos.y + buffer_y * 2.0)) / instruction_size).floor(),
    );
    if mouse_selection_pos.x >= 0.0
        && mouse_selection_pos.x < 5.0
        && mouse_selection_pos.y >= 0.0
        && mouse_selection_pos.y < options_height as f32
    {
        let index = mouse_selection_pos.x as usize
            + mouse_selection_pos.y as usize * options_width as usize;
        if index < avalible_instructions.len() {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                brain_edit.selected_instruction.instruction_type = InstructionType::Blank;
                let _ = sound_handle.play_raw(
                    Decoder::new(BufReader::new(File::open("Assets/button_up.wav").unwrap()))
                        .unwrap()
                        .convert_samples(),
                );
            }
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                brain_edit.selected_instruction.instruction_type = avalible_instructions[index].1;
                let _ = sound_handle.play_raw(
                    Decoder::new(BufReader::new(
                        File::open("Assets/button_down.wav").unwrap(),
                    ))
                    .unwrap()
                    .convert_samples(),
                );
            }
        }
    }
}
