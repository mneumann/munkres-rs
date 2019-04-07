use crate::Position;
use fixedbitset::FixedBitSet;

#[derive(Debug)]
pub struct Coverage {
    n: usize,
    /// A bit is set, if the row is uncovered.
    uncovered_rows: FixedBitSet,
    /// A bit is set, if the column is uncovered.
    uncovered_columns: FixedBitSet,
}

impl Coverage {
    #[inline]
    pub fn n(&self) -> usize {
        self.n
    }

    pub fn new(n: usize) -> Coverage {
        assert!(n > 0);

        // We start with all bits set (all rows/columns) uncovered.
        let mut all_rows_uncovered = FixedBitSet::with_capacity(n);
        all_rows_uncovered.set_range(.., true);

        // We can simply clone the rows as we work on square matrices
        let all_columns_uncovered = all_rows_uncovered.clone();

        Coverage {
            n: n,
            uncovered_rows: all_rows_uncovered,
            uncovered_columns: all_columns_uncovered,
        }
    }

    /// Find the first uncovered cell. Iterates in column-major order.
    #[inline]
    pub fn find_uncovered_cell_column_row_order<F>(&self, mut f: F) -> Option<Position>
    where
        F: FnMut(Position) -> bool,
    {
        for column in self.uncovered_columns.ones() {
            for row in self.uncovered_rows.ones() {
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
        !self.uncovered_rows.contains(row)
    }

    #[inline]
    pub fn is_column_covered(&self, column: usize) -> bool {
        debug_assert!(column < self.n());
        !self.uncovered_columns.contains(column)
    }

    #[inline]
    pub fn cover(&mut self, pos: Position) {
        self.cover_row(pos.row);
        self.cover_column(pos.column);
    }

    #[inline]
    pub fn cover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.uncovered_columns.set(column, false);
    }

    #[inline]
    pub fn uncover_column(&mut self, column: usize) {
        debug_assert!(column < self.n());
        self.uncovered_columns.set(column, true);
    }

    #[inline]
    pub fn cover_row(&mut self, row: usize) {
        debug_assert!(row < self.n());
        self.uncovered_rows.set(row, false);
    }

    pub fn clear(&mut self) {
        self.uncovered_rows.set_range(.., true);
        self.uncovered_columns.set_range(.., true);
    }

    pub fn all_uncovered(&self) -> bool {
        self.uncovered_rows.count_ones(..) + self.uncovered_columns.count_ones(..)
            == (self.n + self.n)
    }
}
