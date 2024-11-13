#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::too_many_arguments)]

use draw_brain::*;
use draw_map::*;
use inputs::*;
use instructions::*;
use raylib::prelude::*;
use rodio::{
    source::{SamplesConverter, Source},
    Decoder, OutputStream,
};

mod draw_brain;
mod draw_map;
mod inputs;
mod instructions;

struct World {
    robots: Vec<Robot>,
    map: Map,
}

#[derive(Clone, Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Wall,
    Gap,
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
    Back,
    Direction,
    RotateLeft,
    RotateRight,
    None,
    Blank,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl World {
    fn new(map_width: usize, map_height: usize, robots: Vec<Robot>) -> World {
        World {
            robots,
            map: Map::new(map_width, map_height),
        }
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        let mut map: Vec<Cell> = vec![];
        for _ in 0..(width * height) {
            map.push(Cell::Empty);
        }
        Map {
            width,
            height,
            cells: map,
        }
    }
    fn get_cell_type(&self, x: i32, y: i32) -> Cell {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return Cell::Wall;
        }
        let index = x + y * self.width as i32;
        self.cells.clone()[index as usize]
    }
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
    back_instruction: Texture2D,
    right_instruction: Texture2D,
    left_instruction: Texture2D,
    reader: Texture2D,
    font: WeakFont,
    map: MapAssets,
}

struct MapAssets {
    empty: EmptyAssets,
}

struct EmptyAssets {
    oooo: Texture2D,
    oooi: Texture2D,
    ooio: Texture2D,
    ooii: Texture2D,
    oioo: Texture2D,
    oioi: Texture2D,
    oiio: Texture2D,
    oiii: Texture2D,
    iooo: Texture2D,
    iooi: Texture2D,
    ioio: Texture2D,
    ioii: Texture2D,
    iioo: Texture2D,
    iioi: Texture2D,
    iiio: Texture2D,
    iiii: Texture2D,
}

struct BrainEdit {
    pos: Vector2,
    id: Option<usize>,
    size: f32,
    scale: f32,
    selected_instruction: Instruction,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .msaa_4x()
        .vsync()
        .title("Robotery")
        .build();

    let (_stream, sound_handle) = OutputStream::try_default().unwrap();
    let mut world = World::new(
        4,
        4,
        vec![Robot::new(
            (0, 0),
            Rotation::Up,
            5,
            5,
            vec![
                (5, InstructionType::Direction),
                (5, InstructionType::Move),
                (5, InstructionType::Back),
                (5, InstructionType::RotateLeft),
                (5, InstructionType::RotateRight),
            ],
        )],
    );

    world.map.cells[world.map.width + 1] = Cell::Wall;
    world.map.cells[world.map.width] = Cell::Wall;
    world.map.cells[2] = Cell::Wall;

    let mut brain_edit = BrainEdit {
        pos: Vector2 { x: 100.0, y: 450.0 },
        id: None,
        size: 200.0,
        scale: 2.0,
        selected_instruction: Instruction {
            instruction_type: InstructionType::None,
            rotation: Rotation::Up,
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
        back_instruction: rl
            .load_texture(&thread, "Assets/back_instruction.png")
            .unwrap(),
        right_instruction: rl
            .load_texture(&thread, "Assets/right_instruction.png")
            .unwrap(),
        left_instruction: rl
            .load_texture(&thread, "Assets/left_instruction.png")
            .unwrap(),
        reader: rl.load_texture(&thread, "Assets/reader.png").unwrap(),
        font: rl.get_font_default(),
        map: MapAssets {
            empty: EmptyAssets {
                oooo: rl
                    .load_texture(&thread, "Assets/map/empty/0000.png")
                    .unwrap(),
                oooi: rl
                    .load_texture(&thread, "Assets/map/empty/0001.png")
                    .unwrap(),
                ooio: rl
                    .load_texture(&thread, "Assets/map/empty/0010.png")
                    .unwrap(),
                ooii: rl
                    .load_texture(&thread, "Assets/map/empty/0011.png")
                    .unwrap(),
                oioo: rl
                    .load_texture(&thread, "Assets/map/empty/0100.png")
                    .unwrap(),
                oioi: rl
                    .load_texture(&thread, "Assets/map/empty/0101.png")
                    .unwrap(),
                oiio: rl
                    .load_texture(&thread, "Assets/map/empty/0110.png")
                    .unwrap(),
                oiii: rl
                    .load_texture(&thread, "Assets/map/empty/0111.png")
                    .unwrap(),
                iooo: rl
                    .load_texture(&thread, "Assets/map/empty/1000.png")
                    .unwrap(),
                iooi: rl
                    .load_texture(&thread, "Assets/map/empty/1001.png")
                    .unwrap(),
                ioio: rl
                    .load_texture(&thread, "Assets/map/empty/1010.png")
                    .unwrap(),
                ioii: rl
                    .load_texture(&thread, "Assets/map/empty/1011.png")
                    .unwrap(),
                iioo: rl
                    .load_texture(&thread, "Assets/map/empty/1100.png")
                    .unwrap(),
                iioi: rl
                    .load_texture(&thread, "Assets/map/empty/1101.png")
                    .unwrap(),
                iiio: rl
                    .load_texture(&thread, "Assets/map/empty/1110.png")
                    .unwrap(),
                iiii: rl
                    .load_texture(&thread, "Assets/map/empty/1111.png")
                    .unwrap(),
            },
        },
    };
    let update_dt = 0.5;
    let mut time_since_last_step = 0.0;

    let mut stepping = false;
    let mut read_next = true;
    while !rl.window_should_close() {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let dt = rl.get_frame_time();

        let mouse_pos = rl.get_mouse_position();

        if stepping {
            time_since_last_step += dt;
        }

        (read_next, time_since_last_step) =
            update_robots(&mut world, read_next, time_since_last_step, update_dt);

        stepping = inputs(
            &mut rl,
            &mut world,
            &assets,
            mouse_pos,
            &mut brain_edit,
            &sound_handle,
            stepping,
        );

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(20, 20, 20, 255));

        draw_board(
            &mut d,
            &world,
            &assets,
            Vector2::new(50.0, 50.0),
            500.0,
            1.0,
        );
        if brain_edit.id.is_some() {
            draw_brain(
                &mut d,
                &world.robots[brain_edit.id.unwrap()].brain,
                brain_edit.pos,
                brain_edit.size,
                &assets,
                mouse_pos,
                &brain_edit.selected_instruction,
                brain_edit.scale,
            );
        }
    }
}
