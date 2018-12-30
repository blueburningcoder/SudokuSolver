//! Gameboard controller.

use piston::input::GenericEvent;

use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    pub cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                match key {
                    Key::D0 => self.gameboard.set(ind, 0),
                    Key::D1 => self.gameboard.set(ind, 1),
                    Key::D2 => self.gameboard.set(ind, 2),
                    Key::D3 => self.gameboard.set(ind, 3),
                    Key::D4 => self.gameboard.set(ind, 4),
                    Key::D5 => self.gameboard.set(ind, 5),
                    Key::D6 => self.gameboard.set(ind, 6),
                    Key::D7 => self.gameboard.set(ind, 7),
                    Key::D8 => self.gameboard.set(ind, 8),
                    Key::D9 => self.gameboard.set(ind, 9),
                    Key::NumPad0 => self.gameboard.set(ind, 0),
                    Key::NumPad1 => self.gameboard.set(ind, 1),
                    Key::NumPad2 => self.gameboard.set(ind, 2),
                    Key::NumPad3 => self.gameboard.set(ind, 3),
                    Key::NumPad4 => self.gameboard.set(ind, 4),
                    Key::NumPad5 => self.gameboard.set(ind, 5),
                    Key::NumPad6 => self.gameboard.set(ind, 6),
                    Key::NumPad7 => self.gameboard.set(ind, 7),
                    Key::NumPad8 => self.gameboard.set(ind, 8),
                    Key::NumPad9 => self.gameboard.set(ind, 9),
                    Key::Right => self.select(ind[0], ind[1], 1, 0),
                    Key::Down => self.select(ind[0], ind[1], 0, 1),
                    Key::Left => self.select(ind[0], ind[1], -1, 0),
                    Key::Up => self.select(ind[0], ind[1], 0, -1),
                    _ => ()
                }
            }
            self.gameboard.autosolve();
        }
    }

    fn select(&mut self, x: usize, y: usize, dx: i32, dy: i32) {
        if x == 0 && dx < 0 {
            self.selected_cell = Some([8, y])
        } else if y == 0 && dy < 0 {
            self.selected_cell = Some([x, 8])
        } else {
            let x = (x as i32 + dx) as usize;
            let y = (y as i32 + dy) as usize;
            self.selected_cell = Some([x % 9, y % 9])
        }
        // self.selected_cell.map_or(0, |ind| self.gameboard.get(ind))
    }
}
