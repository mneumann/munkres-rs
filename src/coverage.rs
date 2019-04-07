use crate::Position;
use fixedbitset::FixedBitSet;

#[derive(Debug)]
pub struct Coverage {
    n: usize,
    /// A bit is set, if the row is covered.
    covered_rows: FixedBitSet,
    /// A bit is set, if the column is covered.
    covered_columns: FixedBitSet,
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
            covered_rows: FixedBitSet::with_capacity(n),
            covered_columns: FixedBitSet::with_capacity(n),
        }
    }

    /// find a single uncovered (row, column) pair. Iterates in column, row order.
    #[inline]
    pub fn find_uncovered_column_row<F>(&self, mut f: F) -> Option<Position>
    where
        F: FnMut(Position) -> bool,
    {
        let n = self.n();

        for column in 0..n {
            if self.is_column_covered(column) {
                continue;
            }
            for row in 0..n {
                if self.is_row_covered(row) {
                    continue;
                }

                let pos = Position { row, column };
                if f(pos) {
                    return Some(pos);
                }
            }
        }

        return None;
    }

    /// iterates over all uncovered (row, column) pairs in row, column order
    #[inline]
    pub fn iter_uncovered_row_column<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        let n = self.n();

        for row in 0..n {
            if self.is_row_covered(row) {
                continue;
            }

            for column in 0..n {
                if self.is_column_covered(column) {
                    continue;
                }

                f(Position { row, column });
            }
        }
    }

    /// iterates over all uncovered (row, column) pairs in row, column order, and set covered if f returns true.
    #[inline]
    pub fn iter_uncovered_row_column_and_cover<F>(&mut self, mut f: F)
    where
        F: FnMut(Position) -> bool,
    {
        let n = self.n();

        for row in 0..n {
            if self.is_row_covered(row) {
                continue;
            }

            'column: for column in 0..n {
                if self.is_column_covered(column) {
                    continue;
                }

                let pos = Position { row, column };
                if f(pos) {
                    self.cover(pos);
                    // the complete row is covered! break the loop!
                    break 'column;
                }
            }
        }
    }

    #[inline]
    pub fn is_row_covered(&self, row: usize) -> bool {
        debug_assert!(row < self.n());
        self.covered_rows.contains(row)
    }

    #[inline]
    pub fn is_column_covered(&self, column: usize) -> bool {
        debug_assert!(column < self.n());
        self.covered_columns.contains(column)
    }

    #[inline]
    pub fn cover(&mut self, pos: Position) {
        self.cover_row(pos.row);
        self.cover_column(pos.column);
    }

    #[inline]
    pub fn cover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.covered_columns.set(column, true);
    }

    #[inline]
    pub fn uncover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.covered_columns.set(column, false);
    }

    #[inline]
    pub fn cover_row(&mut self, row: usize) {
        debug_assert!(row < self.n());
        self.covered_rows.set(row, true);
    }

    pub fn clear(&mut self) {
        self.covered_rows.clear();
        self.covered_columns.clear();
    }

    pub fn all_uncovered(&self) -> bool {
        is_bitset_clear(&self.covered_rows) && is_bitset_clear(&self.covered_columns)
    }
}

fn is_bitset_clear(bitset: &FixedBitSet) -> bool {
    for elm in bitset.as_slice().iter() {
        if *elm != 0 {
            return false;
        }
    }
    return true;
}
