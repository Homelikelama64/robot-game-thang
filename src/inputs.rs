use raylib::prelude::*;

use crate::{Assets, BrainEdit, InstructionType, Map, Rotation};

pub fn inputs(
    rl: &mut RaylibHandle,
    map: &mut Map,
    assets: &Assets,
    mouse_pos: Vector2,
    brain_edit: &mut BrainEdit,
    stepping: bool,
) -> bool {
    let mut stepping = stepping;
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        stepping = !stepping;
    }
    if brain_edit.id.is_some() {
        brain(rl, map, assets, mouse_pos, brain_edit);
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
    map: &mut Map,
    assets: &Assets,
    mouse_pos: Vector2,
    brain_edit: &mut BrainEdit,
) {
    let brain = &mut map.robots[brain_edit.id.unwrap()].brain;
    let width = brain_edit.size - assets.brain_corner.width as f32 * 2.0;
    let instruction_size = width / brain.width as f32;
    let mouse_brain_pos = Vector2::new(
        ((mouse_pos.x - (brain_edit.pos.x + assets.brain_corner.width as f32)) / instruction_size)
            .floor(),
        (-((mouse_pos.y - (brain_edit.pos.y - assets.brain_corner.width as f32))
            / instruction_size))
            .floor(),
    );
    let instruction_count =
        brain.get_instruction_count(brain_edit.selected_instruction.instruction_type);
    if instruction_count != 0
        && brain.in_bounds((mouse_brain_pos.x as i32, mouse_brain_pos.y as i32))
        && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
    {
        brain.instructions
            [mouse_brain_pos.x as usize + mouse_brain_pos.y as usize * brain.width as usize] =
            brain_edit.selected_instruction;
    }

    let avalible_instructions = brain.get_avalible_instructions();
    let options_width = 5;
    let options_height = (avalible_instructions.len() as f32 / options_width as f32).ceil() as i32;
    let instruction_size =
        (brain_edit.size - assets.brain_corner.width as f32 * 2.0) / options_width as f32;
    let mouse_selection_pos = Vector2::new(
        ((mouse_pos.x - (brain_edit.pos.x + assets.brain_corner.width as f32)) / instruction_size)
            .floor(),
        ((mouse_pos.y - (brain_edit.pos.y + assets.brain_corner.height as f32 * 3.0))
            / instruction_size)
            .floor(),
    );
    if mouse_selection_pos.x >= 0.0
        && mouse_selection_pos.x < 5.0
        && mouse_selection_pos.y >= 0.0
        && mouse_selection_pos.y < options_height as f32
    {
        let index = mouse_selection_pos.x as usize
            + mouse_selection_pos.y as usize * options_width as usize;
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && index < avalible_instructions.len()
        {
            brain_edit.selected_instruction.instruction_type = avalible_instructions[index].1;
        }
    }
}
