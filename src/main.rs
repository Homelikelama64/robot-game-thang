#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::too_many_arguments)]

use draw_instructions::*;
use draw_robot::*;
use raylib::prelude::*;

mod draw_instructions;
mod draw_robot;

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
    reader: Reader,
}
#[derive(Clone, Copy, Debug)]
struct Reader {
    pos: (i32, i32),
    rotation: Rotation,
}
#[derive(Clone, Copy, Debug)]
struct Instruction {
    instruction_type: InstructionType,
    rotation: Rotation,
}

#[derive(Clone, Copy, Debug)]
enum InstructionType {
    Forwards,
    Direction,
    RotateLeft,
    RotateRight,
    None,
}

#[derive(Clone, Copy, Debug)]
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
    fn new(pos: (i32, i32), rotation: Rotation, brain_width: u32, brain_height: u32) -> Robot {
        Robot {
            brain: Brain::new(brain_width, brain_height),
            pos,
            rotation,
        }
    }
}

impl Brain {
    fn new(width: u32, height: u32) -> Brain {
        let mut instructions: Vec<Instruction> = vec![];
        for _ in 0..width * height {
            instructions.push(Instruction {
                instruction_type: InstructionType::None,
                rotation: Rotation::Up,
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
        }
    }
    fn get_instruction(&self, pos: (i32, i32)) -> &Instruction {
        if !self.in_bounds(pos) {
            return &Instruction {
                instruction_type: InstructionType::None,
                rotation: Rotation::Up,
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
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut map = Map::new(10, 10, vec![Robot::new((0, 0), Rotation::Up, 5, 5)]);

    let mut selected = Instruction {
        instruction_type: InstructionType::Direction,
        rotation: Rotation::Right,
    };

    let blank_asset = rl.load_texture(&thread, "Assets/blank.png").unwrap();
    let direction_asset = rl.load_texture(&thread, "Assets/direction.png").unwrap();
    let forwards_asset = rl.load_texture(&thread, "Assets/forwards.png").unwrap();
    let clear_asset = rl.load_texture(&thread, "Assets/clear.png").unwrap();
    let robot_asset = rl.load_texture(&thread, "Assets/robot.png").unwrap();
    let reader_asset = rl.load_texture(&thread, "Assets/reader.png").unwrap();
    let rotate_left_asset = rl.load_texture(&thread, "Assets/rotate_left.png").unwrap();
    let rotate_right_asset = rl.load_texture(&thread, "Assets/rotate_right.png").unwrap();

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
                        InstructionType::Forwards => match robot.rotation {
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

        let mut y_off = buffer;
        for i in 0..map.robots.len() {
            let robot = &mut map.robots[i];
            let brain = &mut robot.brain;

            let scale = (max_brain_area_width - (buffer * 2.0)) / brain.width as f32;

            let brain_pixel_height = brain.height as f32 * scale;

            let mouse_brain_pos = Vector2 {
                x: ((mouse_pos.x - buffer) / scale).floor(),
                y: (-(mouse_pos.y - brain_pixel_height - y_off) / scale).floor(),
            };
            if brain.in_bounds((mouse_brain_pos.x as i32, mouse_brain_pos.y as i32))
                && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            {
                let index = (mouse_brain_pos.x + mouse_brain_pos.y * brain.width as f32) as usize;
                brain.instructions[index] = selected;
            }
            y_off += brain_pixel_height + buffer;
        }

        scrolls += rl.get_mouse_wheel_move();
        if scrolls < -1.0 {
            match selected.rotation {
                Rotation::Up => selected.rotation = Rotation::Right,
                Rotation::Right => selected.rotation = Rotation::Down,
                Rotation::Down => selected.rotation = Rotation::Left,
                Rotation::Left => selected.rotation = Rotation::Up,
            }
            scrolls = 0.0;
        } else if scrolls > 1.0 {
            match selected.rotation {
                Rotation::Up => selected.rotation = Rotation::Left,
                Rotation::Right => selected.rotation = Rotation::Up,
                Rotation::Down => selected.rotation = Rotation::Right,
                Rotation::Left => selected.rotation = Rotation::Down,
            }
            scrolls = 0.0;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            match selected.instruction_type {
                InstructionType::Direction => selected.instruction_type = InstructionType::Forwards,
                InstructionType::Forwards => {
                    selected.instruction_type = InstructionType::RotateLeft;
                }
                InstructionType::RotateLeft => {
                    selected.instruction_type = InstructionType::RotateRight;
                }
                InstructionType::RotateRight => selected.instruction_type = InstructionType::None,
                InstructionType::None => selected.instruction_type = InstructionType::Direction,
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            stepping = !stepping;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(51, 51, 51, 255));

        let map_width = width as f32
            - max_brain_area_width
            - buffer * 2.0
            - (width as f32 - max_brain_area_width) / 3.0;

        let mut y_off = buffer;
        #[expect(unused)]
        let x_off = width as f32 - max_brain_area_width;
        for i in 0..map.robots.len() {

            {
                // Robot Drawing
                draw_robot(
                    &mut d,
                    &map.robots[i],
                    map_width,
                    &map,
                    max_brain_area_width,
                    buffer,
                    width,
                    height,
                    &robot_asset,
                );
            }
            
            let robot = &mut map.robots[i];
            let brain = &mut robot.brain;

            {
                draw_instructions(
                    &mut d,
                    brain,
                    width,
                    height,
                    buffer,
                    max_brain_area_width,
                    y_off,
                    mouse_pos,
                    &forwards_asset,
                    &direction_asset,
                    &blank_asset,
                    &rotate_left_asset,
                    &rotate_right_asset,
                    &clear_asset,
                    &reader_asset,
                    selected,
                );
                y_off += brain.height as f32
                    * ((max_brain_area_width - (buffer * 2.0)) / brain.width as f32)
                    + buffer;
            }
        }
    }
}
