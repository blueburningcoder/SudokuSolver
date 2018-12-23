//! Game board logic.

use either::*;

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
#[derive(Debug, Copy, Clone)]
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
    /// Stores the possibilities for all the cells.
    /// has to be updated regularly.
    pub possible: [[[bool; 9]; SIZE]; SIZE],
}


/// Delta on game boards.
/// This includes changes in cell-occupation and possible values.
/// One Delta is one change, however they are usually in lists.
#[derive(Debug, Copy, Clone)]
struct Delta {
    /// Either a cells value got updated, or the possibilities.
    delta: ([usize; 2], Either<u8, [bool; 9]>),
    /// these are the previous values. Necessary for 'revert'.
    previous: ([usize; 2], Either<u8, [bool; 9]>),
    /// If the delta has been applied already.
    applied: bool,
    /// The index of this delta.
    index: u32,
}


impl Delta {
    /// This is applying a delta to a Gameboard.
    pub fn apply(&mut self, mut g: Gameboard) {
//         let delta = self.delta;
//         self.applied = true;
//         self.previous = (delta.0, delta.1.either(|_| Left(g.get(delta.0)), |_| Right(g.getpossible(delta.0))));
//         let f: Box<Fn(Gameboard)>;
//         f = match delta.1 {
//             Left(v) => Box::new(move |mut g: Gameboard| g.set(delta.0, v)),
//             Right(p) => Box::new(move |mut g: Gameboard| g.setpossible(delta.0, p)),
//         };
//         f(g);
//        let f: Box<Fn(Gameboard)> = delta.1.either(
//            |v| Box::new(move |mut g: Gameboard| g.set(delta.0, v)),
//            |p| Box::new(move |mut g: Gameboard| g.setpossible(delta.0, p))
//            );
//        f(g);
    }

    /// This is reverting the change on the Gameboard.
    pub fn revert(self, mut g: Gameboard) {
    }
}


impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
            possible: [[[true; 9]; SIZE]; SIZE],
        }
    }

    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// Get cell value.
    pub fn get(self, ind: [usize; 2]) -> u8 {
        self.cells[ind[1]][ind[0]]
    }

    /// Set cell value. Returns the old value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
//         let old = self.cells[ind[1]][ind[0]];
        self.cells[ind[1]][ind[0]] = val;

        // self.setpossible(ind, poss);
//         old
    }

    /// Set possible values.
    pub fn setpossible(&mut self, ind: [usize; 2], poss: [bool; 9]) -> u8 {
        let r = self.cells[ind[1]][ind[0]];
        if r != 0 {
            return r;
        }
        self.possible[ind[1]][ind[0]] = poss;
        0
    }

    /// Get possible values.
    pub fn getpossible(self, ind: [usize; 2]) -> [bool; 9] {
        self.possible[ind[1]][ind[0]]
    }

    /// Solve this sudoku.
    pub fn autosolve(&mut self) {
        for i in 0..9 {
            if let Some(deltas) = self.clone().possibleinrow(i) {
            }
            if let Some(deltas) = self.clone().possibleincolum(i) {
            }
            if let Some(deltas) = self.clone().possibleincluster(i) {
            }
        }
    }

    /// return the number missing in this row.
    fn possibleinrow(self, row: u8) -> Option<(usize, usize, u8)> {
        None
    }

    /// return the number missing in this column.
    fn possibleincolum(self, column: u8) -> Option<(usize, usize, u8)> {
        None
    }
 
    /// return the number missing in this cluster.
    fn possibleincluster(self, cluster: u8) -> Option<(usize, usize, u8)> {
        None
    }
}
