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

    /// Creating a new Delta 'object'.
    pub fn new(ind: [usize; 2], new: Either<u8, [bool; 9]>) -> Delta {
        Delta {
            delta: (ind, new),
            previous: (ind, new),
            applied: false,
            index: 0,
        }
    }

    /// This is applying a delta to a Gameboard.
    pub fn apply(&mut self, g: &mut Gameboard) {
        let delta = self.delta;
        self.applied = true;
        self.previous = (delta.0, delta.1.either(
            |_| Left(g.get(delta.0)),
            |_| Right(g.getpossible(delta.0))
            ));
        if let Left(v) = delta.1 {
            g.set(delta.0, v);
        } else if let Right(p) = delta.1 {
            g.setpossible(delta.0, p);
        }
//         let f: Box<Fn(Gameboard)>;
//         f = match delta.1 {
//             Left(v) => Box::new(move |mut g: Gameboard| g.set(delta.0, v)),
//             Right(p) => Box::new(move |mut g: Gameboard| g.setpossible(delta.0, p)),
//         };
//         let f: Box<Fn(Gameboard)> = delta.1.either(
//             |v| Box::new(move |mut g: Gameboard| g.set(delta.0, v)),
//             |p| Box::new(move |mut g: Gameboard| g.setpossible(delta.0, p))
//             );
//         f(g);
    }

/*
    /// This is reverting the change on the Gameboard.
    pub fn revert(self, g: &mut Gameboard) {
        if !self.applied {
            return;
        }

    }
*/

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
        self.cells[ind[1]][ind[0]] = val;

        if val == 0 {
            self.resetpossible();
        }
    }

    /// Set possible values.
    pub fn setpossible(&mut self, ind: [usize; 2], poss: [bool; 9]) {
        let r = self.cells[ind[1]][ind[0]];
        if r != 0 {
            return;
        }
        for i in 0..9 {
            self.possible[ind[1]][ind[0]][i] =
                self.possible[ind[1]][ind[0]][i] && poss[i];
        }
    }


    /// Resetting the possible values to everything possible.
    /// Necessary after removing numbers.
    fn resetpossible(&mut self) {
        self.possible = [[[true; 9]; SIZE]; SIZE];
    }

    /// Get possible values.
    pub fn getpossible(self, ind: [usize; 2]) -> [bool; 9] {
        self.possible[ind[1]][ind[0]]
    }

    /// Trying to solve this sudoku, using forward checking and arc-consistency.
    /// Not capable of solving it if there's multiple solutions.
    pub fn autosolve(&mut self) {
        let mut deltas = Vec::new();
        // first, g
        for i in 0..9 {
            deltas.extend(self.clone().possibleinrow(i));
            deltas.extend(self.clone().possibleincol(i));
            deltas.extend(self.clone().possibleincluster(i));
        }
        for delta in deltas.iter_mut() {
            delta.apply(self);
            // println!("applied delta: {:?}", delta);
        }
        self.setonlypossible();
        // deltas.iter().for_each(|d: &mut Delta| d.apply(&mut self));
    }

    /// Proposes updated Deltas regarding the possibilities of this row.
    fn possibleinrow(self, row: usize) -> Vec<Delta> {
        let mut possible = [true; 9];
        let mut possibles = 9;
        for i in 0..9 {
            let a = self.cells[row][i];
            if a > 0 {
                possible[(a-1) as usize] = false;
                possibles -= 1;
            }
        }

        if possibles == 0 {
            return Vec::new();
        }

        let mut d = Vec::<Delta>::new();
        for i in 0..9 {
            let a = self.cells[row][i];
            if a == 0 {
                let mut newposs = [true; 9];
                for k in 0..9 {
                    newposs[k] = self.possible[row][i][k] && possible[k];
                }
                d.push(Delta::new([i, row], Right(newposs)));
            }
        }
        d
    }

    /// Proposes updated Deltas regarding the possibilities of this column.
    fn possibleincol(self, col: usize) -> Vec<Delta> {
        let mut possible = [false; 9];
        let mut possibles = 9;
        for i in 0..9 {
            let a = self.cells[i][col];
            if a > 0 {
                possible[(a-1) as usize] = false;
                possibles -= 1;
            }
        }

        if possibles == 0 {
            return Vec::new();
        }

        let mut d = Vec::<Delta>::new();
        for i in 0..9 {
            let a = self.cells[i][col];
            if a == 0 {
                let mut newposs = [true; 9];
                for k in 0..9 {
                    newposs[k] = self.possible[i][col][k] || possible[k];
                }
                if (newposs != self.possible[i][col]) {
                    d.push(Delta::new([col, i], Right(newposs)));
                }
            }
        }
        d
    }

    /// Return the number missing in this cluster.
    fn possibleincluster(self, cluster: usize) -> Vec<Delta> {
        Vec::new()
    }

    /// If there's only one number left to be possible, set this number.
    fn setonlypossible(&mut self) {

    }
}
