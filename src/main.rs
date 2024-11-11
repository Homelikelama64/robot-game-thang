#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::too_many_arguments)]

use draw_brain::*;
use inputs::*;
use raylib::prelude::*;
use rodio::{
    source::{SamplesConverter, Source},
    Decoder, OutputStream,
};
use std::fs::File;
use std::io::BufReader;

mod draw_brain;
mod inputs;

#[derive(Clone, Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    robots: Vec<Robot>,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Clone, Debug)]
struct Robot {
    brain: Brain,
    pos: (i32, i32),
    rotation: Rotation,
}

#[derive(Clone, Debug)]
struct Brain {
    width: u32,
    height: u32,
    instructions: Vec<Instruction>,
    total_instructions: Vec<(usize, InstructionType)>,
    reader: Reader,
}
#[derive(Clone, Copy, Debug)]
struct Reader {
    pos: (i32, i32),
    rotation: Rotation,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    instruction_type: InstructionType,
    rotation: Rotation,
    edit: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionType {
    Move,
    Direction,
    RotateLeft,
    RotateRight,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl Map {
    fn new(width: usize, height: usize, robots: Vec<Robot>) -> Map {
        let mut map: Vec<Cell> = vec![];
        for _ in 0..(width * height) {
            map.push(Cell::Empty);
        }
        Map {
            width,
            height,
            cells: map,
            robots,
        }
    }
}

fn get_cell_type(x: i32, y: i32, width: usize, height: usize, cells: &[Cell]) -> Cell {
    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return Cell::Wall;
    }
    let index = x + y * width as i32;
    cells[index as usize]
}

impl Robot {
    fn new(
        pos: (i32, i32),
        rotation: Rotation,
        brain_width: u32,
        brain_height: u32,
        total_instructions: Vec<(usize, InstructionType)>,
    ) -> Robot {
        Robot {
            brain: Brain::new(brain_width, brain_height, total_instructions),
            pos,
            rotation,
        }
    }
}

impl Brain {
    fn new(width: u32, height: u32, total_instructions: Vec<(usize, InstructionType)>) -> Brain {
        let mut instructions: Vec<Instruction> = vec![];
        for _ in 0..width * height {
            instructions.push(Instruction {
                instruction_type: InstructionType::None,
                rotation: Rotation::Up,
                edit: true,
            });
        }
        Brain {
            width,
            height,
            instructions,
            reader: Reader {
                pos: (0, 0),
                rotation: Rotation::Up,
            },
            total_instructions,
        }
    }
    fn get_instruction(&self, pos: (i32, i32)) -> &Instruction {
        if !self.in_bounds(pos) {
            return &Instruction {
                instruction_type: InstructionType::None,
                rotation: Rotation::Up,
                edit: false,
            };
        }
        let index = (pos.0 + pos.1 * self.width as i32) as usize;
        &self.instructions[index]
    }
    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        if pos.0 >= 0 && pos.0 < self.width as i32 && pos.1 >= 0 && pos.1 < self.height as i32 {
            return true;
        }
        false
    }
    fn get_avalible_instructions(&self) -> Vec<(usize, InstructionType)> {
        let mut total_instructions = self.total_instructions.clone();
        total_instructions.insert(0, (1, InstructionType::None));
        for avalible_instruction in &mut total_instructions {
            if avalible_instruction.1 != InstructionType::None {
                for instruction in &self.instructions {
                    let other_instruction_type = instruction.instruction_type;
                    if avalible_instruction.1 == other_instruction_type
                        && avalible_instruction.0 != 0
                    {
                        avalible_instruction.0 -= 1;
                    }
                }
            }
        }
        total_instructions.retain(|instruction| instruction.0 != 0);
        total_instructions
    }
    fn get_instruction_count(&self, instruction: InstructionType) -> usize {
        let avalible_instructions = self.get_avalible_instructions();
        let mut count = 0;
        for avalible_instruction in avalible_instructions {
            if avalible_instruction.1 == instruction {
                count = avalible_instruction.0;
            }
        }
        count
    }
}

struct Assets {
    brain_edge: Texture2D,
    brain_corner: Texture2D,
    blank_instruction: Texture2D,
    up_instruction: Texture2D,
    down_instruction: Texture2D,
    direction_instruction: Texture2D,
    move_instruction: Texture2D,
    right_instruction: Texture2D,
    left_instruction: Texture2D,
}
struct BrainEdit {
    pos: Vector2,
    id: Option<usize>,
    size: f32,
    selected_instruction: Instruction,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let (stream, sound_handle) = OutputStream::try_default().unwrap();
    let mut map = Map::new(
        10,
        10,
        vec![Robot::new(
            (0, 0),
            Rotation::Up,
            10,
            10,
            vec![
                (5, InstructionType::Direction),
                (5, InstructionType::Move),
                (5, InstructionType::RotateLeft),
                (5, InstructionType::RotateRight),
            ],
        )],
    );

    let mut brain_edit = BrainEdit {
        pos: Vector2 { x: 100.0, y: 650.0 },
        id: Some(0),
        size: 600.0,
        selected_instruction: Instruction {
            instruction_type: InstructionType::None,
            rotation: Rotation::Right,
            edit: true,
        },
    };

    let assets = Assets {
        brain_edge: rl
            .load_texture(&thread, "Assets/brain_boarder.png")
            .unwrap(),
        brain_corner: rl.load_texture(&thread, "Assets/brain_corner.png").unwrap(),
        blank_instruction: rl
            .load_texture(&thread, "Assets/blank_instruction.png")
            .unwrap(),
        up_instruction: rl
            .load_texture(&thread, "Assets/up_instruction.png")
            .unwrap(),
        down_instruction: rl
            .load_texture(&thread, "Assets/down_instruction.png")
            .unwrap(),
        direction_instruction: rl
            .load_texture(&thread, "Assets/direction_instruction.png")
            .unwrap(),
        move_instruction: rl
            .load_texture(&thread, "Assets/move_instruction.png")
            .unwrap(),
        right_instruction: rl
            .load_texture(&thread, "Assets/right_instruction.png")
            .unwrap(),
        left_instruction: rl
            .load_texture(&thread, "Assets/left_instruction.png")
            .unwrap(),
    };
    let update_dt = 0.5;
    let mut time_since_last_step = 0.0;

    let mut scrolls = 0.0;

    let mut stepping = false;
    let mut read_next = true;
    while !rl.window_should_close() {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let dt = rl.get_frame_time();

        let mouse_pos = rl.get_mouse_position();

        let buffer = if height < width {
            height as f32
        } else {
            width as f32
        } / 50.0;
        let mut max_brain_area_width = width as f32 / 3.0;

        let mut brains_height = buffer;
        for robot in &map.robots {
            let brain = &robot.brain;
            let scale = (max_brain_area_width - (buffer * 2.0)) / brain.width as f32;

            brains_height += brain.height as f32 * scale + buffer;
        }

        if brains_height > height as f32 {
            max_brain_area_width /= (brains_height) / (height as f32);
        }

        if stepping {
            time_since_last_step += dt;
        }
        while time_since_last_step > update_dt {
            for robot in &mut map.robots {
                let brain = &mut robot.brain;
                let instruction = brain.get_instruction(brain.reader.pos);
                if read_next {
                    match instruction.instruction_type {
                        InstructionType::Move => match robot.rotation {
                            Rotation::Up => match get_cell_type(
                                robot.pos.0,
                                robot.pos.1 + 1,
                                map.width,
                                map.height,
                                &map.cells,
                            ) {
                                Cell::Empty => robot.pos.1 += 1,
                                Cell::Wall => {}
                            },
                            Rotation::Right => match get_cell_type(
                                robot.pos.0 + 1,
                                robot.pos.1,
                                map.width,
                                map.height,
                                &map.cells,
                            ) {
                                Cell::Empty => robot.pos.0 += 1,
                                Cell::Wall => {}
                            },
                            Rotation::Down => match get_cell_type(
                                robot.pos.0,
                                robot.pos.1 - 1,
                                map.width,
                                map.height,
                                &map.cells,
                            ) {
                                Cell::Empty => robot.pos.1 -= 1,
                                Cell::Wall => {}
                            },
                            Rotation::Left => match get_cell_type(
                                robot.pos.0 - 1,
                                robot.pos.1,
                                map.width,
                                map.height,
                                &map.cells,
                            ) {
                                Cell::Empty => robot.pos.0 -= 1,
                                Cell::Wall => {}
                            },
                        },
                        InstructionType::Direction => {
                            brain.reader.rotation = instruction.rotation;
                        }
                        InstructionType::None => {}
                        InstructionType::RotateLeft => {
                            robot.rotation = match robot.rotation {
                                Rotation::Up => Rotation::Left,
                                Rotation::Right => Rotation::Up,
                                Rotation::Down => Rotation::Right,
                                Rotation::Left => Rotation::Down,
                            }
                        }
                        InstructionType::RotateRight => {
                            robot.rotation = match robot.rotation {
                                Rotation::Up => Rotation::Right,
                                Rotation::Right => Rotation::Down,
                                Rotation::Down => Rotation::Left,
                                Rotation::Left => Rotation::Up,
                            }
                        }
                    }
                }
                match brain.reader.rotation {
                    Rotation::Up => {
                        if brain.in_bounds((brain.reader.pos.0, brain.reader.pos.1 + 1))
                            && !matches!(
                                brain
                                    .get_instruction((brain.reader.pos.0, brain.reader.pos.1 + 1))
                                    .instruction_type,
                                InstructionType::None
                            )
                        {
                            brain.reader.pos.1 += 1;
                            read_next = true;
                        } else {
                            read_next = false;
                        }
                    }
                    Rotation::Right => {
                        if brain.in_bounds((brain.reader.pos.0 + 1, brain.reader.pos.1))
                            && !matches!(
                                brain
                                    .get_instruction((brain.reader.pos.0 + 1, brain.reader.pos.1))
                                    .instruction_type,
                                InstructionType::None
                            )
                        {
                            brain.reader.pos.0 += 1;
                            read_next = true;
                        } else {
                            read_next = false;
                        }
                    }
                    Rotation::Down => {
                        if brain.in_bounds((brain.reader.pos.0, brain.reader.pos.1 - 1))
                            && !matches!(
                                brain
                                    .get_instruction((brain.reader.pos.0, brain.reader.pos.1 - 1))
                                    .instruction_type,
                                InstructionType::None
                            )
                        {
                            brain.reader.pos.1 -= 1;
                            read_next = true;
                        } else {
                            read_next = false;
                        }
                    }
                    Rotation::Left => {
                        if brain.in_bounds((brain.reader.pos.0 - 1, brain.reader.pos.1))
                            && !matches!(
                                brain
                                    .get_instruction((brain.reader.pos.0 - 1, brain.reader.pos.1))
                                    .instruction_type,
                                InstructionType::None
                            )
                        {
                            brain.reader.pos.0 -= 1;
                            read_next = true;
                        } else {
                            read_next = false;
                        }
                    }
                }
            }
            time_since_last_step -= update_dt;
        }
        stepping = inputs(
            &mut rl,
            &mut map,
            &assets,
            mouse_pos,
            &mut brain_edit,
            &sound_handle,
            stepping,
        );

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(51, 51, 51, 255));

        if brain_edit.id.is_some() {
            draw_brain(
                &mut d,
                &map.robots[brain_edit.id.unwrap()].brain,
                brain_edit.pos,
                brain_edit.size,
                &assets,
                mouse_pos,
                &brain_edit.selected_instruction,
                1.0,
            );
        }
    }
}
