use bit_vec::BitVec;

#[derive(Debug)]
pub struct Coverage {
    rows: BitVec,
    cols: BitVec,
}

impl Coverage {
    // XXX: Is this needed?
    pub fn n(&self) -> usize {
        let n1 = self.rows.len();
        let n2 = self.cols.len();
        assert!(n1 == n2);
        return n1;
    }

    pub fn new(n: usize) -> Coverage {
        Coverage {
            rows: BitVec::from_elem(n, false),
            cols: BitVec::from_elem(n, false),
        }
    }

    #[inline]
    pub fn is_row_covered(&self, row: usize) -> bool {
        self.rows[row]
    }

    #[inline]
    pub fn is_col_covered(&self, col: usize) -> bool {
        self.cols[col]
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
