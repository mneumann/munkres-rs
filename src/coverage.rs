use fixedbitset::FixedBitSet;

#[derive(Debug)]
pub struct Coverage {
    n: usize,
    rows: FixedBitSet,
    cols: FixedBitSet,
}

impl Coverage {
    pub fn n(&self) -> usize { self.n }

    pub fn new(n: usize) -> Coverage {
        Coverage {
            n: n,
            rows: FixedBitSet::with_capacity(n),
            cols: FixedBitSet::with_capacity(n),
        }
    }

    #[inline]
    /// find a single uncovered (row, col) pair. Iterates in col, row order.
    pub fn find_uncovered_col_row<F>(&self, mut f: F) -> Option<(usize, usize)>
        where F: FnMut((usize, usize)) -> bool {

        let n = self.n();

        for col in 0..n {
            if self.is_col_covered(col) {
                continue;
            }
            for row in 0..n {
                if self.is_row_covered(row) {
                    continue;
                }

                let pos = (row, col);
                if f(pos) {
                    return Some(pos);
                }
            }
        }

        return None;
    }

    #[inline]
    /// iterates over all uncovered (row, col) pairs in row, col order
    pub fn iter_uncovered_row_col<F>(&self, mut f: F)
        where F: FnMut((usize, usize)) {
        let n = self.n();

        for row in 0..n {
            if self.is_row_covered(row) {
                continue;
            }

            for col in 0..n {
                if self.is_col_covered(col) {
                    continue;
                }

                f((row, col));
            }
        }
    }

    #[inline]
    /// iterates over all uncovered (row, col) pairs in row, col order, and set covered if f returns true.
    pub fn iter_uncovered_row_col_and_cover<F>(&mut self, mut f: F)
        where F: FnMut((usize, usize)) -> bool {
        let n = self.n();

        for row in 0..n {
            if self.is_row_covered(row) {
                continue;
            }

            for col in 0..n {
                if self.is_col_covered(col) {
                    continue;
                }

                let pos = (row, col);
                if f(pos) {
                    self.cover(pos);
                }
            }
        }
    }

    #[inline]
    pub fn is_row_covered(&self, row: usize) -> bool {
        self.rows.contains(row)
    }

    #[inline]
    pub fn is_col_covered(&self, col: usize) -> bool {
        self.cols.contains(col)
    }

    #[inline]
    pub fn cover(&mut self, pos: (usize, usize)) {
        match pos {
            (row, col) => {
                self.cover_row(row);
                self.cover_col(col);
            }
        }
    }

    #[inline]
    pub fn cover_col(&mut self, col: usize) {
        self.cols.set(col, true);
    }

    #[inline]
    pub fn uncover_col(&mut self, col: usize) {
        self.cols.set(col, false);
    }

    #[inline]
    pub fn cover_row(&mut self, row: usize) {
        self.rows.set(row, true);
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.cols.clear();
    }
}
