use raylib::prelude::*;

use crate::{Assets, Map};

pub fn draw_board(d: &mut RaylibDrawHandle, map: &Map, assets: &Assets, pos: Vector2, width: f32, scale: f32) {
    let width = width * scale;
    let cell_size = width / map.width as f32;
    for (i, cell) in map.cells.iter().enumerate() {
        let grid_pos = Vector2::new((i % map.width) as f32, (i / map.width) as f32);

        let pos = Vector2::new(
            pos.x + grid_pos.x * cell_size,
            pos.y + grid_pos.y * cell_size,
        );
        match cell {
            crate::Cell::Empty => {}
            crate::Cell::Wall => {
                d.draw_rectangle_rec(
                    Rectangle {
                        x: pos.x,
                        y: pos.y,
                        width: cell_size,
                        height: cell_size,
                    },
                    Color::new(100, 100, 100, 255),
                );
            }
        }
    }
}
