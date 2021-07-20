use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Cell {
    x_pos: i32,
    y_pos: i32,
    is_bomb: bool,
    total_bomb: i32,
    revealed: bool,
    flaged: bool,
    bomb_triggered: bool,
}

impl Cell {
    fn new(x_pos: i32, y_pos: i32, is_bomb: bool) -> Self {
        Self {
            x_pos,
            y_pos,
            is_bomb,
            total_bomb: 0,
            revealed: false,
            flaged: false,
            bomb_triggered: false,
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
enum BoardState {
    Pristine = 0,
    Undecided,
    Won,
    Lost,
}

#[wasm_bindgen]
pub struct Board {
    row: i32,
    column: i32,
    cell_size: i32,
    cells: Vec<Vec<Cell>>,
    total_flags_left: i32,
    state: BoardState,
}

impl Board {
    fn bomb_near_cell(&self, i: i32, j: i32, search_end: bool) -> i32 {
        if i < 0 || j < 0 || i >= self.row || j >= self.column {
            return 0;
        }
        if self.cells[i as usize][j as usize].is_bomb {
            return 1;
        }
        if search_end {
            return 0;
        }

        self.bomb_near_cell(i - 1, j - 1, true)
            + self.bomb_near_cell(i - 1, j, true)
            + self.bomb_near_cell(i - 1, j + 1, true)
            + self.bomb_near_cell(i, j - 1, true)
            + self.bomb_near_cell(i, j + 1, true)
            + self.bomb_near_cell(i + 1, j - 1, true)
            + self.bomb_near_cell(i + 1, j, true)
            + self.bomb_near_cell(i + 1, j + 1, true)
    }

    fn calculate_total_bomb(&mut self) {
        for i in 0..self.row as usize {
            for j in 0..self.column as usize {
                self.cells[i][j].total_bomb = self.bomb_near_cell(i as i32, j as i32, false)
            }
        }
    }

    fn _reveal_cell(&mut self, x_coordinate: i32, y_coordinate: i32) -> bool {
        if x_coordinate < 0
            || y_coordinate < 0
            || x_coordinate >= self.column
            || y_coordinate >= self.row
            || self.cells[y_coordinate as usize][x_coordinate as usize].revealed
            || self.cells[y_coordinate as usize][x_coordinate as usize].flaged
        {
            return true;
        }

        if self.cells[y_coordinate as usize][x_coordinate as usize].is_bomb {
            return false;
        }

        self.cells[y_coordinate as usize][x_coordinate as usize].revealed = true;

        if self.cells[y_coordinate as usize][x_coordinate as usize].total_bomb == 0 {
            self.reveal_cell(x_coordinate - 1, y_coordinate - 1);
            self.reveal_cell(x_coordinate, y_coordinate - 1);
            self.reveal_cell(x_coordinate + 1, y_coordinate - 1);
            self.reveal_cell(x_coordinate - 1, y_coordinate);
            self.reveal_cell(x_coordinate + 1, y_coordinate);
            self.reveal_cell(x_coordinate - 1, y_coordinate + 1);
            self.reveal_cell(x_coordinate, y_coordinate + 1);
            self.reveal_cell(x_coordinate + 1, y_coordinate + 1);
        }

        true
    }

    fn reveal_all_bombs(&mut self) {
        for i in 0..self.row as usize {
            for j in 0..self.column as usize {
                if self.cells[i][j].is_bomb {
                    self.cells[i][j].revealed = true;
                }
            }
        }
    }

    fn place_bombs(&mut self, first_x_coordinate: i32, first_y_coordinate: i32) {
        let mut total_bombs = 0;

        while total_bombs != self.total_flags_left {
            for i in 0..self.row {
                for j in 0..self.column {
                    if js_sys::Math::random() < 0.1
                        && (i != first_y_coordinate || j != first_x_coordinate)
                        && !self.cells[i as usize][j as usize].is_bomb
                        && total_bombs < self.total_flags_left
                        && !is_adjacent_cell(first_x_coordinate, first_y_coordinate, j, i)
                    {
                        self.cells[i as usize][j as usize].is_bomb = true;
                        total_bombs += 1;
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
// Public methods to use from Javascript.
impl Board {
    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

    pub fn get_cell_size(&self) -> i32 {
        self.cell_size
    }

    pub fn get_total_flags_left(&self) -> i32 {
        self.total_flags_left
    }

    pub fn get_state(&self) -> u8 {
        self.state as u8
    }

    pub fn flat_cells_details(&self) -> Vec<i32> {
        // cell flat structure
        // [ x_pos, y_pos, is_bomb, total_bomb, revealed, flaged, bomb_triggered]
        let mut cells_details = Vec::with_capacity((self.row * self.column * 7) as usize);

        for i in 0..self.row as usize {
            for j in 0..self.column as usize {
                cells_details.push(self.cells[i][j].x_pos);
                cells_details.push(self.cells[i][j].y_pos);
                cells_details.push(self.cells[i][j].is_bomb as i32);
                cells_details.push(self.cells[i][j].total_bomb);
                cells_details.push(self.cells[i][j].revealed as i32);
                cells_details.push(self.cells[i][j].flaged as i32);
                cells_details.push(self.cells[i][j].bomb_triggered as i32);
            }
        }

        cells_details
    }

    pub fn reveal_cell(&mut self, x_coordinate: i32, y_coordinate: i32) {
        if self.state == BoardState::Pristine {
            self.place_bombs(x_coordinate, y_coordinate);
            self.calculate_total_bomb();
            self.state = BoardState::Undecided;
        } else if self.state != BoardState::Undecided {
            return;
        }

        if self._reveal_cell(x_coordinate, y_coordinate) == false {
            self.state = BoardState::Lost;
            self.reveal_all_bombs();
            self.cells[y_coordinate as usize][x_coordinate as usize].bomb_triggered = true;
        } else if self.state == BoardState::Undecided {
            for i in 0..self.row as usize {
                for j in 0..self.column as usize {
                    if !self.cells[i][j].revealed && !self.cells[i][j].is_bomb {
                        return;
                    }
                }
            }
            self.state = BoardState::Won;
        }
    }

    pub fn toggle_flag(&mut self, x_coordinate: usize, y_coordinate: usize) {
        if self.cells[y_coordinate][x_coordinate].revealed || self.state > BoardState::Undecided {
            return;
        }

        if self.cells[y_coordinate][x_coordinate].flaged {
            self.cells[y_coordinate][x_coordinate].flaged = false;
            self.total_flags_left += 1;
        } else {
            self.cells[y_coordinate][x_coordinate].flaged = true;
            self.total_flags_left -= 1;
        }
    }

    pub fn new(row: i32, column: i32, cell_size: i32, total_bombs: i32) -> Self {
        let mut cells = Vec::with_capacity(row as usize);
        for i in 0..row {
            let mut column_cells = Vec::with_capacity(column as usize);
            for j in 0..column {
                column_cells.push(Cell::new(j * cell_size, i * cell_size, false));
            }
            cells.push(column_cells);
        }

        Self {
            row,
            column,
            cell_size,
            cells,
            total_flags_left: total_bombs,
            state: BoardState::Pristine,
        }
    }
}

fn is_adjacent_cell(ref_x: i32, ref_y: i32, target_x: i32, target_y: i32) -> bool {
    (target_x == ref_x - 1 && target_y == ref_y - 1)
        || (target_x == ref_x && target_y == ref_y - 1)
        || (target_x == ref_x + 1 && target_y == ref_y - 1)
        || (target_x == ref_x - 1 && target_y == ref_y)
        || (target_x == ref_x + 1 && target_y == ref_y)
        || (target_x == ref_x - 1 && target_y == ref_y + 1)
        || (target_x == ref_x && target_y == ref_y + 1)
        || (target_x == ref_x + 1 && target_y == ref_y + 1)
}
