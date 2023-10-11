use macroquad::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead
}

const X_OFFSET: i32 = 5;
const Y_OFFSET: i32 = 5;

const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 10;

const CELL_WIDTH: f32 = 20.0;
const CELL_HEIGHT: f32 = 20.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: GRID_WIDTH * CELL_WIDTH as i32 + X_OFFSET,
        window_height: GRID_HEIGHT * CELL_HEIGHT as i32 + Y_OFFSET,
        // Other configuration options go here
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cells = vec![CellState::Dead; (GRID_WIDTH * GRID_HEIGHT) as usize];

    loop {
        clear_background(BLACK);

        for (index, cell) in cells.iter().enumerate() {
            let x_pos = index % 10;
            let y_pos = index / 10;

            if *cell == CellState::Alive {
                draw_rectangle(
                    0.0 + (x_pos as f32 * CELL_WIDTH) + X_OFFSET as f32,
                    0.0 + (y_pos as f32 * CELL_HEIGHT) + Y_OFFSET as f32,
                    CELL_WIDTH,
                    CELL_HEIGHT,
                    WHITE);
            }
            else {
                draw_rectangle_lines(
                    0.0 + (x_pos as f32 * CELL_WIDTH) + X_OFFSET as f32,
                    0.0 + (y_pos as f32 * CELL_HEIGHT) + Y_OFFSET as f32,
                    CELL_WIDTH,
                    CELL_HEIGHT,
                    2.0,
                    WHITE);
            }
        }

        next_frame().await
    }
}