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
