use crate::{Cell, InstructionType, Map, Rotation, World};

pub fn update_robots(
    world: &mut World,
    read_next: bool,
    time_since_last_step: f32,
    update_dt: f32,
) -> (bool, f32) {
    let mut read_next = read_next;
    let mut time_since_last_step = time_since_last_step;
    while time_since_last_step > update_dt {
        for robot in &mut world.robots {
            let brain = &mut robot.brain;
            let instruction = brain.get_instruction(brain.reader.pos);
            if read_next {
                match instruction.instruction_type {
                    InstructionType::Move => {
                        let pos = match robot.rotation {
                            Rotation::Up => (robot.pos.0, robot.pos.1 + 1),
                            Rotation::Right => (robot.pos.0 + 1, robot.pos.1),
                            Rotation::Down => (robot.pos.0, robot.pos.1 - 1),
                            Rotation::Left => (robot.pos.0 - 1, robot.pos.1),
                        };
                        let cell = world.map.get_cell_type(pos.0, pos.1);
                        match cell {
                            Cell::Empty => robot.pos = pos,
                            Cell::Wall => {}
                            Cell::Gap => {},
                        }
                    }
                    InstructionType::Back => {
                        let pos = match robot.rotation {
                            Rotation::Up => (robot.pos.0, robot.pos.1 - 1),
                            Rotation::Right => (robot.pos.0 - 1, robot.pos.1),
                            Rotation::Down => (robot.pos.0, robot.pos.1 + 1),
                            Rotation::Left => (robot.pos.0 + 1, robot.pos.1),
                        };
                        let cell = world.map.get_cell_type(pos.0, pos.1);
                        match cell {
                            Cell::Empty => robot.pos = pos,
                            Cell::Wall => {}
                            Cell::Gap => {},
                        }
                    }
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
                    InstructionType::Blank => {}
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
    (read_next, time_since_last_step)
}
