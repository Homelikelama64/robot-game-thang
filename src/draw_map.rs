use raylib::prelude::*;

use crate::{Assets, Cell, Map, World};

pub fn draw_board(
    d: &mut RaylibDrawHandle,
    world: &World,
    assets: &Assets,
    pos: Vector2,
    width: f32,
    scale: f32,
) {
    let width = width * scale;
    let cell_size = width / world.map.width as f32;
    dbg!(world.map.cells.len());
    for (i, cell) in world.map.cells.iter().enumerate() {
        let grid_pos = Vector2::new((i % world.map.width) as f32, (i / world.map.width) as f32);

        let pos = Vector2::new(
            pos.x + grid_pos.x * cell_size,
            pos.y + (-grid_pos.y) * cell_size + cell_size * world.map.height as f32,
        );
        draw_cell(
            d,
            assets,
            world,
            (grid_pos.x as i32, grid_pos.y as i32),
            pos,
            cell_size,
            scale,
        );
    }
}

fn draw_cell(
    d: &mut RaylibDrawHandle,
    assets: &Assets,
    world: &World,
    grid_pos: (i32, i32),
    pos: Vector2,
    size: f32,
    scale: f32,
) {
    let center_cell = world.map.get_cell_type(grid_pos.0, grid_pos.1);
    let surrounding_cells = (
        world.map.get_cell_type(grid_pos.0, grid_pos.1 + 1),
        world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1),
        world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1),
        world.map.get_cell_type(grid_pos.0 - 1, grid_pos.1),
    );
    match center_cell {
        crate::Cell::Empty => {
            #[rustfmt::skip]
            let texture = match surrounding_cells {
                (Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty) =>                                             &assets.map.empty.iiii,
                (Cell::Empty, Cell::Empty, Cell::Empty, Cell::Wall | Cell::Gap) =>                                  &assets.map.empty.iiio,
                (Cell::Empty, Cell::Empty, Cell::Wall | Cell::Gap, Cell::Empty) =>                                  &assets.map.empty.iioi,
                (Cell::Empty, Cell::Empty, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap) =>                       &assets.map.empty.iioo,
                (Cell::Empty, Cell::Wall | Cell::Gap, Cell::Empty, Cell::Empty) =>                                  &assets.map.empty.ioii,
                (Cell::Empty, Cell::Wall | Cell::Gap, Cell::Empty, Cell::Wall | Cell::Gap) =>                       &assets.map.empty.ioio,
                (Cell::Empty, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Empty) =>                       &assets.map.empty.iooi,
                (Cell::Empty, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap) =>            &assets.map.empty.iooo,
                (Cell::Wall | Cell::Gap, Cell::Empty, Cell::Empty, Cell::Empty) =>                                  &assets.map.empty.oiii,
                (Cell::Wall | Cell::Gap, Cell::Empty, Cell::Empty, Cell::Wall | Cell::Gap) =>                       &assets.map.empty.oiio,
                (Cell::Wall | Cell::Gap, Cell::Empty, Cell::Wall | Cell::Gap, Cell::Empty) =>                       &assets.map.empty.oioi,
                (Cell::Wall | Cell::Gap, Cell::Empty, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap) =>            &assets.map.empty.oioo,
                (Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Empty, Cell::Empty) =>                       &assets.map.empty.ooii,
                (Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Empty, Cell::Wall | Cell::Gap) =>            &assets.map.empty.ooio,
                (Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Empty) =>            &assets.map.empty.oooi,
                (Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap, Cell::Wall | Cell::Gap) => &assets.map.empty.oooo,
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
                    x: pos.x,
                    y: pos.y,
                    width: size,
                    height: size,
                },
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
        Cell::Wall => {},
        Cell::Gap => {},
    }
}
