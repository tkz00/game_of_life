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
    let mut cells = vec![CellState::Dead; (GRID_WIDTH * GRID_HEIGHT) as usize];

    // Blinker
    cells[1] = CellState::Alive;
    cells[11] = CellState::Alive;
    cells[21] = CellState::Alive;

    // Block
    cells[5] = CellState::Alive;
    cells[6] = CellState::Alive;
    cells[15] = CellState::Alive;
    cells[16] = CellState::Alive;

    // Toad
    cells[47] = CellState::Alive;
    cells[48] = CellState::Alive;
    cells[49] = CellState::Alive;
    cells[56] = CellState::Alive;
    cells[57] = CellState::Alive;
    cells[58] = CellState::Alive;

    // Beacon
    cells[51] = CellState::Alive;
    cells[52] = CellState::Alive;
    cells[61] = CellState::Alive;
    cells[83] = CellState::Alive;
    cells[84] = CellState::Alive;
    cells[74] = CellState::Alive;

    let mut last_update = get_time();
    let update_speed = 0.65;
    
    loop {
        if get_time() - last_update > update_speed {
            // Game logic
            last_update = get_time();

            let mut cells_sheet = cells.clone();
            
            for index in 0..(GRID_WIDTH * GRID_HEIGHT) - 1 {
                let mut alive_neighbours = 0;
                let mut check_positions = vec![];
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let x_pos = index % GRID_WIDTH;
                        let y_pos = index / GRID_WIDTH;
                        let neighbor_x = x_pos as i32 + dx;
                        let neighbor_y = y_pos as i32 + dy;
            
                        if neighbor_x >= 0
                            && neighbor_x < GRID_WIDTH as i32
                            && neighbor_y >= 0
                            && neighbor_y < GRID_HEIGHT as i32
                            && (dx != 0 || dy != 0)
                        {
                            check_positions.push(dx + dy * GRID_WIDTH as i32);
                        }
                    }
                }

                for position in check_positions {
                    if cells[(index + position) as usize] == CellState::Alive {
                        alive_neighbours += 1;
                    }
                }

                if cells[index as usize] == CellState::Alive && !(alive_neighbours == 2 || alive_neighbours == 3) {
                    cells_sheet[index as usize] = CellState::Dead;
                }

                if cells[index as usize] == CellState::Dead && alive_neighbours == 3 {
                    cells_sheet[index as usize] = CellState::Alive;
                }
            }

            cells = cells_sheet;
        }
        
        // Rendering
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