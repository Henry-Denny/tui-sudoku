pub const GRID_SIZE: usize = 9;
const SQUARE_DENSITY: usize = 3;
const SQUARE_SIZE: usize = GRID_SIZE / SQUARE_DENSITY;

pub type CellVal = Option<u32>;

#[derive(Clone, Copy)]
pub struct GridPos {
    pub row: usize,
    pub col: usize,
}

pub struct SudokuGrid {
    grid: [[CellVal; GRID_SIZE]; GRID_SIZE],
    empty_cells: usize,
}

impl SudokuGrid {
    pub fn new() -> Self {
        SudokuGrid {
            grid: [[None; GRID_SIZE]; GRID_SIZE],
            empty_cells: GRID_SIZE * GRID_SIZE
        }
    }

    /// Set the number inside a given cell.
    pub fn set_cell(&mut self, cell: GridPos, num: u32) {
        self.grid[cell.row][cell.col] = Some(num);
        self.empty_cells -= 1;
    }

    /// Remove the number inside a given cell.
    pub fn clear_cell(&mut self, cell: GridPos) {
        self.grid[cell.row][cell.col] = None;
        self.empty_cells += 1;
    }

    /// Empty the sudoku grid.
    pub fn clear_grid(&mut self) {
        for row in self.grid.as_mut() {
            for cell in row {
                *cell = None;
            }
        }
        self.empty_cells = GRID_SIZE * GRID_SIZE;
    }

    /// Get the contents of a given cell.
    pub fn get_cell(self: &Self, cell: GridPos) -> CellVal {
        self.grid[cell.row][cell.col]
    }

    /// Given the current state, solve the grid.
    /// Return value represents whether it was possible to solve.
    pub fn solve(&mut self) -> bool {
        // Perform initial check
        if self.contains_conflicts() {
            return false;
        }
        true
    }

    /// Checks whether the grid has been completely filled in.
    pub fn is_full(self: &Self) -> bool {
        return self.empty_cells == 0;
    }

    pub fn contains_conflicts(self: &Self) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.is_cell_conflicting(GridPos { row, col }) {
                    return true;
                }
            }
        }
        false
    }

    /// Checks whether a certain cell clashes with another.
    pub fn is_cell_conflicting(self: &Self, cell: GridPos) -> bool {
        match self.get_cell(cell) {
            None => false,
            Some(num) => {
                self.is_conflicting_row(num, cell.row) || 
                self.is_conflicting_col(num, cell.col) || 
                self.is_conflicting_square(num, cell.row, cell.col)
            }
        }
    }

    fn is_conflicting_row(self: &Self, num: u32, row: usize) -> bool {
        let mut count = 0;
        for col in 0..GRID_SIZE {
            if let Some(member) = self.get_cell(GridPos { row, col }) {
                if num == member {
                    count += 1;
                    if count >= 2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_conflicting_col(self: &Self, num: u32, col: usize) -> bool {
        let mut count = 0;
        for row in 0..GRID_SIZE {
            if let Some(member) = self.get_cell(GridPos { row, col }) {
                if num == member {
                    count += 1;
                    if count >= 2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_conflicting_square(self: &Self, num: u32, row: usize, col: usize) -> bool {
        let start_row = (row / SQUARE_DENSITY) * SQUARE_SIZE;
        let start_col = (col / SQUARE_DENSITY) * SQUARE_SIZE;
        let mut count = 0;
        for row_off in 0..(SQUARE_SIZE-1) {
            for col_off in 0..(SQUARE_SIZE-1) {
                if let Some(member) = self.get_cell(
                    GridPos {
                        row: start_row + row_off,
                        col: start_col + col_off
                    })
                {
                    if num == member {
                        count += 1;
                        if count >= 2 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}