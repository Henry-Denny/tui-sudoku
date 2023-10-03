use std::error;

use crate::grid::{SudokuGrid, GridPos, GRID_SIZE};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Sudoku grid state.
    grid: SudokuGrid,
    /// The currently selected grid square
    pub selected: GridPos,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            grid: SudokuGrid::new(),
            selected: GridPos { row: 0, col: 0 }
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_up(&mut self) {
        if self.selected.row > 0 {
            self.selected.row -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected.row < GRID_SIZE - 1 {
            self.selected.row += 1;
        }
    }
    
    pub fn move_left(&mut self) {
        if self.selected.col > 0 {
            self.selected.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.selected.col < GRID_SIZE - 1 {
            self.selected.col += 1;
        }
    }

    pub fn set_cell(&mut self, num: u32) {
        self.grid.set_cell(self.selected, num);
    }

    pub fn clear_cell(&mut self) {
        self.grid.clear_cell(self.selected);
    }

    pub fn reset_grid(&mut self) {
        self.grid.clear_grid();
    }


}
