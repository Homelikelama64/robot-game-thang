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
    let mut textures: Vec<&Texture2D> = vec![];
    match center_cell {
        crate::Cell::Empty => {
            d.draw_rectangle_rec(
                Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: size,
                    height: size,
                },
                Color::new(100, 100, 100, 255),
            );

            match world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1) {
                Cell::Empty => {}
                Cell::Wall => textures.push(&assets.map.empty.shade_edge_bottom),
                Cell::Gap => todo!(),
            }
            match world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) {
                Cell::Empty => {}
                Cell::Wall => textures.push(&assets.map.empty.shade_edge_right),
                Cell::Gap => todo!(),
            }
            match world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1 - 1) {
                Cell::Empty => {}
                Cell::Wall => textures.push(&assets.map.empty.shade_corner_filled),
                Cell::Gap => todo!(),
            }
            if world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1) == Cell::Wall
                && world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1 - 1) == Cell::Empty
            {
                textures.push(&assets.map.empty.shade_corner_bottom)
            }
            if world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) == Cell::Wall
                && world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1 - 1) == Cell::Empty
            {
                textures.push(&assets.map.empty.shade_corner_right)
            }

        }
        Cell::Wall => {
            d.draw_rectangle_rec(
                Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: size,
                    height: size,
                },
                Color::new(128, 128, 128, 255),
            );
            if world.map.get_cell_type(grid_pos.0, grid_pos.1 + 1) != Cell::Wall {
                textures.push(&assets.map.wall.top);
            }
            if world.map.get_cell_type(grid_pos.0 - 1, grid_pos.1) != Cell::Wall {
                textures.push(&assets.map.wall.left);
            }
            if world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) != Cell::Wall {
                textures.push(&assets.map.wall.right);
            }
            if world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1) != Cell::Wall {
                textures.push(&assets.map.wall.bottom);
            }
            if world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) != Cell::Wall
                && world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1) != Cell::Wall
            {
                textures.push(&assets.map.wall.corner_outside);
            }
            if world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) == Cell::Wall
                && world.map.get_cell_type(grid_pos.0 - 1, grid_pos.1) == Cell::Wall
            {
                textures.push(&assets.map.wall.corner_straight);
            }
            if world.map.get_cell_type(grid_pos.0, grid_pos.1 - 1) == Cell::Wall
                && world.map.get_cell_type(grid_pos.0 + 1, grid_pos.1) == Cell::Wall
            {
                textures.push(&assets.map.wall.corner_inside);
            }
        }
        Cell::Gap => {}
    }
    for texture in textures {
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
}
