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
        self.char_from_num(self.cells[ind[1]][ind[0]])
    }

    /// Transforms a number to its character.
    pub fn char_from_num(self, num: u8) -> Option<char> {
        Some(match num {
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
        if self.ispossible(ind, val) {
            println!("setting {} at {:?}.", val, ind);
            self.cells[ind[1]][ind[0]] = val;
            self.possible[ind[1]][ind[0]] = [false; 9];
            if val == 0 {
                self.resetpossible();
            }
        }
    }

    /// Checking if this value is possible at this index.
    pub fn ispossible(self, ind: [usize; 2], val: u8) -> bool {
        if val == 0 {
            return true;
        }
        let col = ind[0];
        let row = ind[1];
        let bc = (col / 3) * 3;
        let br = (row / 3) * 3;
        for i in 0..9 {
            if self.cells[row][i] == val ||
                self.cells[i][col] == val ||
                self.cells[br + i / 3][bc + i % 3] == val
            {
                return false;
            }
        }
        true
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
        for i in 0..9 {
            for j in 0..9 {
                if self.cells[i][j] != 0 {
                    self.possible[i][j] = [false; 9];
                    self.possible[i][j][(self.cells[i][j] - 1) as usize] = true;
                }
            }
        }
        let mut deltas = Vec::new();
        for i in 0..9 {
            // forward checking
            deltas.extend(self.clone().possibleinrow(i));
            deltas.extend(self.clone().possibleincol(i));
            deltas.extend(self.clone().possibleincluster(i));
        }
        for delta in deltas.iter_mut() {
            delta.apply(self);
            // println!("applied delta: {:?}", delta);
        }
    }

    /// Get possible values.
    pub fn getpossible(self, ind: [usize; 2]) -> [bool; 9] {
        self.possible[ind[1]][ind[0]]
    }

    /// Trying to solve this sudoku, using forward checking and arc-consistency.
    /// Not capable of solving it if there's multiple solutions.
    pub fn autosolve(&mut self) {
        self.setonlypossible();
        let mut deltas = Vec::new();
        // first, g
        for i in 0..9 {
            // forward checking
            deltas.extend(self.clone().possibleinrow(i));
            deltas.extend(self.clone().possibleincol(i));
            deltas.extend(self.clone().possibleincluster(i));
        }
        for delta in deltas.iter_mut() {
            delta.apply(self);
        }
        deltas = Vec::new();
        for i in 0..9 {
            // Arc Consistency Tier 1: Check if only possibility
            deltas.extend(self.clone().onlypossinrow(i));
            deltas.extend(self.clone().onlypossincol(i));
            deltas.extend(self.clone().onlypossincluster(i));
            // Arc Consistency Tier 2: Test if possibilities are realistically
            // 'used' to begin with
        }
        for delta in deltas.iter_mut() {
            delta.apply(self);
        }
        // if self.setonlypossible() {
        //      self.autosolve()
        // }
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
                if newposs != self.possible[row][i] {
                    d.push(Delta::new([i, row], Right(newposs)));
                }
            }
        }
        d
    }

    /// Proposes updated Deltas regarding the possibilities of this column.
    fn possibleincol(self, col: usize) -> Vec<Delta> {
        let mut possible = [true; 9];
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
                    newposs[k] = self.possible[i][col][k] && possible[k];
                }
                if newposs != self.possible[i][col] {
                    d.push(Delta::new([col, i], Right(newposs)));
                }
            }
        }
        d
    }

    /// Return the number missing in this cluster.
    fn possibleincluster(self, cluster: usize) -> Vec<Delta> {
        let mut possible = [true; 9];
        let mut possibles = 9;
        // getting the base indices of this cluster
        let bc = (cluster % 3) * 3;
        let br = (cluster / 3) * 3;
        for i in 0..9 {
            let a = self.cells[br + i / 3][bc + i % 3];
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
            let a = self.cells[br + i / 3][bc + i % 3];
            if a == 0 {
                let mut newposs = [true; 9];
                for k in 0..9 {
                    newposs[k] = self.possible[br + i / 3][bc + i % 3][k] && possible[k];
                }
                if newposs != self.possible[br + i / 3][bc + i % 3] {
                    d.push(Delta::new([bc + i % 3, br + i / 3], Right(newposs)));
                }
            }
        }
        // println!("with number: {}: {},{}    p: {}, {:?}", self.cells[br + i / 3][bc + i % 3], br + i / 3, bc + i % 3, possibles, possible);
        d
    }

    /// Testing if this col has a possibility that is only in one cell.
    fn onlypossincol(self, row: usize) -> Vec<Delta> {
        let mut sums = [[false; 9]; SIZE];

        for i in 0..9 {
            if self.cells[row][i] == 0 {
                for j in 0..9 {
                    sums[j][i] = self.possible[row][i][j];
                }
            }
        }
        let mut d = Vec::new();
        for i in 0..9 {
            let s: Vec<usize> = sums[i].iter().enumerate().filter(|(_i, b)| **b).map(|(i, _b)| i).collect();
            if s.len() == 1 {
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([s[0], row], Right(redposs)));
            }
        }
        d
    }

    /// Testing if this row has a possibility that is only in one cell.
    fn onlypossinrow(self, col: usize) -> Vec<Delta> {
        let mut sums = [[false; 9]; SIZE];

        for i in 0..9 {
            if self.cells[i][col] == 0 {
                for j in 0..9 {
                    sums[j][i] = self.possible[i][col][j];
                }
            }
        }
        let mut d = Vec::new();
        for i in 0..9 {
            let s: Vec<usize> = sums[i].iter().enumerate().filter(|(_i, b)| **b).map(|(i, _b)| i).collect();
            if s.len() == 1 {
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([col, s[0]], Right(redposs)));
            }
        }
        d
    }

    /// Testing if this cell has a possibility that is only in one cell.
    fn onlypossincluster(self, cluster: usize) -> Vec<Delta> {
        let mut sums = [[false; 9]; SIZE];

        let bc = (cluster % 3) * 3;
        let br = (cluster / 3) * 3;
        for i in 0..9 {
            if self.cells[br + i / 3][bc + i % 3] == 0 {
                for j in 0..9 {
                    sums[j][i] = self.possible[br + i / 3][bc + i % 3][j];
                }
            }
        }
        let mut d = Vec::new();
        for i in 0..9 {
            let s: Vec<usize> = sums[i].iter().enumerate().filter(|(_i, b)| **b).map(|(i, _b)| i).collect();
            if s.len() == 1 {
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([bc + s[0] % 3, br + s[0] / 3], Right(redposs)));
            }
        }
        d
    }

    /// If there's only one number left to be possible, set this number.
    fn setonlypossible(&mut self) -> bool {
        let mut changedsth = false;
        println!("new round\n\n\n");
        for i in 0..9 {
            for j in 0..9 {
                let s: Vec<usize> = self.possible[i][j].iter().enumerate().filter(|(_i, b)| **b).map(|(i, _b)| i + 1).collect();
                // let d: Vec<usize> = s.iter().map(|i| i + 1).collect();
                println!("amount of possible at {}, {}: {:?}", j, i, s);
                if s.len() == 1 && self.cells[i][j] == 0 {
                    self.set([j, i], s[0] as u8);
                    changedsth = true;
                }
            }
        }
        changedsth
    }
}
