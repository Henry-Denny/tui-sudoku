const GRID_SIZE: usize = 9;

pub type CellVal = Option<u32>;

pub struct SudokuGrid {
    grid: [[CellVal; GRID_SIZE]; GRID_SIZE],
}

impl SudokuGrid {
    /// Set the number inside a given cell.
    pub fn set_cell_number(&mut self, row: usize, col: usize, num: u32) {
        self.grid[row][col] = Some(num);
    }

    /// Remove the number inside a given cell.
    pub fn clear_cell(&mut self, row: usize, col: usize) {
        self.grid[row][col] = None;
    }

    /// Empty the sudoku grid.
    pub fn clear_grid(&mut self) {
        for row in self.grid.as_mut() {
            for cell in row {
                *cell = None;
            }
        }
    }

    /// Get the contents of a given cell.
    pub fn get_cell(&mut self, row: usize, col: usize) -> CellVal {
        self.grid[row][col]
    }

    /// Given the current state, solve the grid.
    /// Return value represents whether it was possible to solve.
    pub fn solve(&mut self) -> bool {
        false
    }
}