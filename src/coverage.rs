use crate::Position;
use fixedbitset::FixedBitSet;

#[derive(Debug)]
pub struct Coverage {
    n: usize,
    rows: FixedBitSet,
    columns: FixedBitSet,
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
            columns: FixedBitSet::with_capacity(n),
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
        self.rows.contains(row)
    }

    #[inline]
    pub fn is_column_covered(&self, column: usize) -> bool {
        debug_assert!(column < self.n());
        self.columns.contains(column)
    }

    #[inline]
    pub fn cover(&mut self, pos: Position) {
        self.cover_row(pos.row);
        self.cover_column(pos.column);
    }

    #[inline]
    pub fn cover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.columns.set(column, true);
    }

    #[inline]
    pub fn uncover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.columns.set(column, false);
    }

    #[inline]
    pub fn cover_row(&mut self, row: usize) {
        debug_assert!(row < self.n());
        self.rows.set(row, true);
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.columns.clear();
    }

    pub fn is_clear(&self) -> bool {
        is_bitset_clear(&self.rows) && is_bitset_clear(&self.columns)
    }
}
