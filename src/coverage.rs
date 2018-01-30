use fixedbitset::FixedBitSet;

#[derive(Debug)]
pub struct Coverage {
    n: usize,
    rows: FixedBitSet,
    cols: FixedBitSet,
}

fn is_bitset_clear(bitset: &FixedBitSet) -> bool {
    for elm in bitset.as_slice().iter() {
        if *elm != 0 {
            return false;
        }
    }
    return true;
}

impl Coverage {
    #[inline]
    pub fn n(&self) -> usize {
        self.n
    }

    pub fn new(n: usize) -> Coverage {
        assert!(n > 0);
        Coverage {
            n: n,
            rows: FixedBitSet::with_capacity(n),
            cols: FixedBitSet::with_capacity(n),
        }
    }

    /// find a single uncovered (row, col) pair. Iterates in col, row order.
    #[inline]
    pub fn find_uncovered_col_row<F>(&self, mut f: F) -> Option<(usize, usize)>
    where
        F: FnMut((usize, usize)) -> bool,
    {
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

    /// iterates over all uncovered (row, col) pairs in row, col order
    #[inline]
    pub fn iter_uncovered_row_col<F>(&self, mut f: F)
    where
        F: FnMut((usize, usize)),
    {
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

    /// iterates over all uncovered (row, col) pairs in row, col order, and set covered if f returns true.
    #[inline]
    pub fn iter_uncovered_row_col_and_cover<F>(&mut self, mut f: F)
    where
        F: FnMut((usize, usize)) -> bool,
    {
        let n = self.n();

        for row in 0..n {
            if self.is_row_covered(row) {
                continue;
            }

            'col: for col in 0..n {
                if self.is_col_covered(col) {
                    continue;
                }

                let pos = (row, col);
                if f(pos) {
                    self.cover(pos);
                    // the complete row is covered! break the loop!
                    break 'col;
                }
            }
        }
    }

    #[inline]
    pub fn is_row_covered(&self, row: usize) -> bool {
        debug_assert!(row < self.n());
        self.rows.contains(row)
    }

    #[inline]
    pub fn is_col_covered(&self, col: usize) -> bool {
        debug_assert!(col < self.n());
        self.cols.contains(col)
    }

    #[inline]
    pub fn cover(&mut self, pos: (usize, usize)) {
        let (row, col) = pos;
        self.cover_row(row);
        self.cover_col(col);
    }

    #[inline]
    pub fn cover_col(&mut self, col: usize) {
        debug_assert!(col < self.n());
        self.cols.set(col, true);
    }

    #[inline]
    pub fn uncover_col(&mut self, col: usize) {
        debug_assert!(col < self.n());
        self.cols.set(col, false);
    }

    #[inline]
    pub fn cover_row(&mut self, row: usize) {
        debug_assert!(row < self.n());
        self.rows.set(row, true);
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.cols.clear();
    }

    pub fn is_clear(&self) -> bool {
        is_bitset_clear(&self.rows) && is_bitset_clear(&self.cols)
    }
}
