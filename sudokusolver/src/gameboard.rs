//! Game board logic.

use either::*;

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
#[derive(Debug, Copy, Clone, Default)]
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
    /// Stores the possibilities for all the cells.
    /// has to be updated regularly.
    pub possible: [[[bool; 9]; SIZE]; SIZE],
    /// If the value has been set manually
    pub manual: [[bool; SIZE]; SIZE],
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
        self.previous = (
            delta.0,
            delta
                .1
                .either(|_| Left(g.get(delta.0)), |_| Right(g.getpossible(delta.0))),
        );
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
            manual: [[false; SIZE]; SIZE],
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

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if self.ispossible(ind, val) {
            self.cells[ind[1]][ind[0]] = val;
            self.possible[ind[1]][ind[0]] = [false; 9];
            if val == 0 {
                self.resetpossible();
            }
        }
    }

    /// Set cell value manually.
    pub fn setmanually(&mut self, ind: [usize; 2], val: u8) {
        if self.ispossible(ind, val) {
            self.cells[ind[1]][ind[0]] = val;
            self.manual[ind[1]][ind[0]] = true;
            self.possible[ind[1]][ind[0]] = [false; 9];
            if val == 0 {
                self.resetpossible();
                self.manual[ind[1]][ind[0]] = false;
            }
        }
    }

    /// reset all fields that have not been set manually.
    pub fn reset_manual(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if !self.manual[i][j] {
                    self.possible[i][j] = [true; 9];
                    self.cells[i][j] = 0;
                }
            }
        }
    }

    /// Checking if this value is possible at this index.
    pub fn ispossible(self, ind: [usize; 2], val: u8) -> bool {
        if val == 0 {
            return true;
        } else if self.cells[ind[1]][ind[0]] != 0 {
            return false;
        }
        let col = ind[0];
        let row = ind[1];
        let bc = (col / 3) * 3;
        let br = (row / 3) * 3;
        for i in 0..9 {
            if self.cells[row][i] == val
                || self.cells[i][col] == val
                || self.cells[br + i / 3][bc + i % 3] == val
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
            self.possible[ind[1]][ind[0]][i] = self.possible[ind[1]][ind[0]][i] && poss[i];
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
            if self.clone().check_delta_changing(*delta) {
                delta.apply(self);
            }
        }
    }

    /// Get possible values.
    pub fn getpossible(self, ind: [usize; 2]) -> [bool; 9] {
        self.possible[ind[1]][ind[0]]
    }

    /// Trying to solve this sudoku, using forward checking and arc-consistency.
    /// Not capable of solving it if there's multiple solutions.
    pub fn autosolve(&mut self, cont: bool) {
        let mut first = true;
        let mut changedsth = [false; 9];
        let mut updated = false;

        while {
            let mut deltas = Vec::new();
            let indices = Gameboard::get_update_indices(changedsth, first);
            changedsth = [false; 9];


            if updated || first {
                for &i in indices.iter() {
                    // forward checking
                    deltas.extend(self.clone().possibleinrow(i));
                    deltas.extend(self.clone().possibleincol(i));
                    deltas.extend(self.clone().possibleincluster(i));
                }

                for delta in deltas.iter_mut() {
                    if self.clone().check_delta_changing(*delta) {
                        delta.apply(self);
                        let loc = delta.delta.0;
                        let cluster = loc[0] / 3 + (loc[1] / 3) * 3;
                        changedsth[cluster] = true;
                    }
                }

                // let deltas: Vec<&Delta> = deltas
                //     .iter()
                //     .filter(|&d| self.clone().check_delta_changing(*d))
                //     .collect();
                // deltas
                //     .map(|d| d.apply(self))
                //     .collect::<Vec<()>>();

                deltas = Vec::new();
            }
            first = false;


            for &i in indices.iter() {
                // Arc Consistency Tier 1: Check if it is the only possibility in
                // a row/col/cluster and set it accordingly.

                // Tier 2a: detect if only possible 'row' or 'col' in cluster, and
                // exclude possibilities in other clusters
                // (implemented)
                //
                // Example (possibilities):
                // 0 0 0 | 0 0 0 | 0 5 0
                // 0 0 0 | 0 5 5 | 0 5 0
                // 5 5 5 | 0 5 5 | 0 0 5
                // After:
                // 0 0 0 | 0 0 0 | 0 5 0
                // 0 0 0 | 0 5 5 | 0 5 0
                // 5 5 5 | 0 X X | 0 0 X
                //
                // test from being inside the cluster if these are the same row/col
                // modify this row/col then in excluding it in the other clusters.

                // Tier 2b: detect one row/col being excluded (number wise) in two
                // clusters in one line (no diagonal). exclude the other
                // (implemented)
                //
                // Example (possibilities):
                // 0 0 0 | 0 0 0 | 5 5 0
                // 0 0 5 | 0 5 0 | 0 5 0
                // 0 5 5 | 0 5 0 | 0 0 5
                // After:
                // 0 0 0 | 0 0 0 | 5 5 0
                // 0 0 5 | 0 5 0 | 0 X 0
                // 0 5 5 | 0 5 0 | 0 0 X
                //
                // test from row/col if this is within the same cluster
                // modify this cluster then in excluding it in the other rows/cols.

                // Tier 3: detect a case in which three possibilities aren't
                // actually needed, and exclude the third. There might even be a
                // more general case than this.
                // (missing)
                //
                // Example (numbers):
                // 0 0 0 | 0 0 0 | 0 0 0
                // 0 0 0 | 0 0 0 | 0 0 5
                // 0 0 0 | 0 0 0 | 0 0 0
                // ---------------------
                // 0 5 0 | 0 0 0 | 0 0 0
                // 0 0 0 | 0 0 0 | 0 0 0
                // 0 0 0 | 0 0 0 | 0 0 0
                // ---------------------
                // 0 0 0 | 0 0 0 | 0 0 0
                // 0 0 0 | 0 0 5 | 0 0 0
                // 0 0 0 | 0 0 0 | 0 0 0
                //
                // Usually there is then in the other fields enough numbers to make
                // in each of them either only two or three possibilities, it is
                // sufficient for only one of them to only have two possibilities
                // in a cluster.
                //
                // test: backtrack both possibilities and exclude the not actually
                // needed fields.

                deltas.extend(self.clone().onlypossinrow(i));
                deltas.extend(self.clone().onlypossincol(i));
                deltas.extend(self.clone().onlypossincluster(i));
            }
            for delta in deltas.iter_mut() {
                if self.clone().check_delta_changing(*delta) {
                    delta.apply(self);
                    let loc = delta.delta.0;
                    let cluster = loc[0] / 3 + (loc[1] / 3) * 3;
                    changedsth[cluster] = true;
                }
            }
            updated = self.setonlypossible();
            (updated || changedsth.iter().any(|&v| v)) && cont
        } {}
    }

    /// Proposes updated Deltas regarding the possibilities of this row.
    fn possibleinrow(self, row: usize) -> Vec<Delta> {
        let mut possible = [true; 9];
        let mut possibles = 9;
        for i in 0..9 {
            let a = self.cells[row][i];
            if a > 0 {
                possible[(a - 1) as usize] = false;
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
                possible[(a - 1) as usize] = false;
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
                possible[(a - 1) as usize] = false;
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
        d
    }

    /// Tier 1: Testing if this row has a possibility that is only in one cell.
    fn onlypossinrow(self, row: usize) -> Vec<Delta> {
        let mut sums = [[false; 9]; SIZE];

        for i in 0..9 {
            if self.cells[row][i] == 0 {
                for (j, rows) in sums.iter_mut().enumerate() {
                    rows[i] = self.possible[row][i][j];
                // for j in 0..9 {
                    // sums[j][i] = self.possible[row][i][j];
                }
            }
        }
        let mut d = Vec::new();
        for i in 0..9 {
            let s: Vec<usize> = sums[i]
                .iter()
                .enumerate()
                .filter(|(_i, b)| **b)
                .map(|(i, _b)| i)
                .collect();
            // s[X] is the col.
            if s.len() == 1 {
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([s[0], row], Right(redposs)));
            } else if s.len() == 2 {
                if s[0] / 3 == s[1] / 3 {
                    // same row and cluster.
                    let cluster = s[0] / 3 + (row / 3) * 3;
                    d.extend(self.excludenumberincluster_exceptrow(cluster, row, i));
                }
            } else if s.len() == 3 {
                if s[0] / 3 == s[1] / 3 && s[1] / 3 == s[2] / 3 {
                    // same row and cluster.
                    let cluster = s[0] / 3 + (row / 3) * 3;
                    d.extend(self.excludenumberincluster_exceptrow(cluster, row, i));
                }
            }
        }
        d
    }

    /// Tier 1: Testing if this col has a possibility that is only in one cell.
    fn onlypossincol(self, col: usize) -> Vec<Delta> {
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
            let s: Vec<usize> = sums[i]
                .iter()
                .enumerate()
                .filter(|(_i, b)| **b)
                .map(|(i, _b)| i)
                .collect();
            // s[X] is the row.
            if s.len() == 1 {
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([col, s[0]], Right(redposs)));
            } else if s.len() == 2 {
                if s[0] / 3 == s[1] / 3 {
                    // same col and cluster.
                    let cluster = col / 3 + (s[0] / 3) * 3;
                    d.extend(self.excludenumberincluster_exceptcol(cluster, col, i));
                }
            } else if s.len() == 3 {
                if s[0] / 3 == s[1] / 3 && s[1] / 3 == s[2] / 3 {
                    // same col and cluster.
                    let cluster = col / 3 + (s[0] / 3) * 3;
                    d.extend(self.excludenumberincluster_exceptcol(cluster, col, i));
                }
            }
        }
        d
    }

    /// Tier 1: Testing if this cell has a possibility that is only in one cell.
    /// Tier 2a: Testing if (2-3) only-possibilities are in one row/col and
    /// excluding other possibilities based on this.
    fn onlypossincluster(self, cluster: usize) -> Vec<Delta> {
        let mut sums = [[false; 9]; SIZE];

        let bc = (cluster % 3) * 3; // base colum of cluster
        let br = (cluster / 3) * 3; // base row of cluster
        for i in 0..9 {
            if self.cells[br + i / 3][bc + i % 3] == 0 {
                for (j, row) in sums.iter_mut().enumerate() {
                    row[i] = self.possible[br + i / 3][bc + i % 3][j];
                // for j in 0..9 {
                    // sums[j][i] = self.possible[br + i / 3][bc + i % 3][j];
                }
            }
        }
        let mut d = Vec::new();
        for i in 0..9 {
            // getting the cells where <num> i is actually possible.
            let s: Vec<usize> = sums[i]
                .iter()
                .enumerate()
                .filter(|(_i, b)| **b)
                .map(|(i, _b)| i)
                .collect();
            if s.len() == 1 {
                // if there's only one cell, simply make <num> i the only
                // possible value. This will in the next iteratien make it the
                // value.
                let mut redposs = [false; 9];
                redposs[i] = true;
                d.push(Delta::new([bc + s[0] % 3, br + s[0] / 3], Right(redposs)));
            } else if s.len() == 2 {
                // two actually have some special cases themselvels.
                if s[0] % 3 == s[1] % 3 {
                    d.extend(self.excludenumberincol_exceptcluster(s[0], cluster, i));
                } else if s[0] / 3 == s[1] / 3 {
                    d.extend(self.excludenumberinrow_exceptcluster(s[0], cluster, i));
                }
            } else if s.len() == 3 {
                // if all three of them are in the same row or colum
                if s[0] % 3 == s[1] % 3 && s[1] % 3 == s[2] % 3 {
                    d.extend(self.excludenumberincol_exceptcluster(s[0], cluster, i));
                } else if s[0] / 3 == s[1] / 3 && s[1] / 3 == s[2] / 3 {
                    d.extend(self.excludenumberinrow_exceptcluster(s[0], cluster, i));
                }
            }
        }
        d
    }

    /// Excluding a number in a row except in this cluster.
    fn excludenumberinrow_exceptcluster(
        self,
        row: usize,
        cluster: usize,
        number: usize,
    ) -> Vec<Delta> {
        let bc = (cluster % 3) * 3; // base colum of cluster
        let br = (cluster / 3) * 3; // base row of cluster
        let mut d = Vec::new();
        let mut redposs = [true; 9];
        redposs[number] = false;
        // same row. now we can exclude other clusters
        // possibilities for this number in this row.
        for j in 0..9 {
            // s[X] % 3 is the additional col
            // s[X] / 3 is the additional row
            // additional to bc and br in this cluster
            // excluding the own cluster ofc
            if j / 3 == bc / 3 {
                continue;
            }
            d.push(Delta::new([j, br + row / 3], Right(redposs)));
        }
        d
    }

    /// Excluding a number in a colum except in this cluster.
    fn excludenumberincol_exceptcluster(
        self,
        col: usize,
        cluster: usize,
        number: usize,
    ) -> Vec<Delta> {
        let bc = (cluster % 3) * 3; // base colum of cluster
        let br = (cluster / 3) * 3; // base row of cluster
        let mut d = Vec::new();
        let mut redposs = [true; 9];
        redposs[number] = false;
        // same col. now we can exclude other clusters
        // possibilities for this number in this col.
        for j in 0..9 {
            // s[X] % 3 is the additional col
            // s[X] / 3 is the additional row
            // additional to bc and br in this cluster
            // excluding the own cluster ofc
            if j / 3 == br / 3 {
                continue;
            }
            d.push(Delta::new([bc + col % 3, j], Right(redposs)));
        }
        d
    }

    /// Excluding a number in a cluster except for one row.
    fn excludenumberincluster_exceptrow(
        self,
        cluster: usize,
        row: usize,
        number: usize,
    ) -> Vec<Delta> {
        let bc = (cluster % 3) * 3; // base colum of cluster
        let br = (cluster / 3) * 3; // base row of cluster
        let mut d = Vec::new();
        let mut redposs = [true; 9];
        redposs[number] = false;
        // iterate through cluster and exclude this row in setting something not possible.
        for i in 0..9 {
            if i / 3 == row % 3 {
                continue;
            }
            d.push(Delta::new([bc + i % 3, br + i / 3], Right(redposs)));
        }

        d
    }

    /// Excluding a number in a cluster except for one colum.
    fn excludenumberincluster_exceptcol(
        self,
        cluster: usize,
        col: usize,
        number: usize,
    ) -> Vec<Delta> {
        let bc = (cluster % 3) * 3; // base colum of cluster
        let br = (cluster / 3) * 3; // base row of cluster
        let mut d = Vec::new();
        let mut redposs = [true; 9];
        redposs[number] = false;
        // iterate through cluster and exclude this col in setting something not possible.
        for i in 0..9 {
            if i % 3 == col % 3 {
                continue;
            }
            d.push(Delta::new([bc + i % 3, br + i / 3], Right(redposs)));
        }

        d
    }

    /// If there's only one number left to be possible, set this number.
    fn setonlypossible(&mut self) -> bool {
        let mut changedsth = false;
        for i in 0..9 {
            for j in 0..9 {
                let s: Vec<usize> = self.possible[i][j]
                    .iter()
                    .enumerate()
                    .filter(|(_i, b)| **b)
                    .map(|(i, _b)| i + 1)
                    .collect();
                // let d: Vec<usize> = s.iter().map(|i| i + 1).collect();
                if s.len() == 1 && self.cells[i][j] == 0 {
                    self.set([j, i], s[0] as u8);
                    changedsth = true;
                }
            }
        }
        changedsth
    }

    fn getleastamount(&mut self, n: usize) -> Vec<(usize, usize)> {
        self.possible
            .iter()
            .enumerate()
            .map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(_j, p)| p.iter().filter(|b| **b).count() == n)
                    .map(|(j, _p)| (i, j))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }

    /// tries to solve this sudoku (at least the next step) using backtracking.
    /// Still buggy.
    pub fn backtracksolver(&mut self) {
        return;
    }

    /// Test if setting this particular value results in a solved board or not.
    fn testbacktrack(&mut self, ind: [usize; 2], val: u8) -> bool {
        let mut test = *self;
        test.set(ind, val);
        test.autosolve(true);
        test.issolved()
    }

    /// Check if this particular Delta would change anything to begin with
    fn check_delta_changing(self, delta: Delta) -> bool {
        let ind = delta.delta.0;

        match delta.delta.1 {
            Left(v) => self.ispossible(ind, v),
            Right(p) => {
                (0..9_usize)
                    .filter(|&i| self.possible[ind[1]][ind[0]][i] && !p[i])
                    .count()
                    > 0
            }
        }

        // for setting the actual value (LEFT):
        // check if it is possible to set.
        // if self.ispossible(ind, val) {
        //
        // for setting new possibilities:
        // check if new 'reduced' possibilities are actually different
        //
        // setpossible:
        // for i in 0..9 {
        //     self.possible[ind[1]][ind[0]][i] = self.possible[ind[1]][ind[0]][i] && poss[i];
        // }
    }

    /// Returns if this Gameboard has been fully solved
    fn issolved(self) -> bool {
        // iterating through all rows (and cells) and return if they are all different from 0.
        self.cells.iter().all(|r| r.iter().all(|&n| n != 0u8))
    }

    /// Returns iterator over indices of cells based on previous changes
    fn get_update_indices(changedsth: [bool; 9], first: bool) -> Vec<usize> {
        if first {
            return vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        }
        let mut indices = vec![];
        for rows in 0..3 {
            if changedsth[rows * 3] || changedsth[rows * 3 + 1] || changedsth[rows * 3 + 2] {
                indices.extend(vec![rows * 3, rows * 3 + 1, rows * 3 + 2]);
            }
        }
        for cols in 0..3 {
            if changedsth[cols] || changedsth[3 + cols] || changedsth[6 + cols] {
                indices.extend(vec![cols, 3 + cols, 6 + cols]);
            }
        }
        indices.sort();
        indices.dedup();

        indices
    }

}



