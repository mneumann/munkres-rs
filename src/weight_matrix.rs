use std::cmp::Ordering;
use super::WeightNum;
use super::square_matrix::SquareMatrix;
use super::coverage::Coverage;

#[derive(Debug)]
// TODO: A WeightMatrix trait
pub struct WeightMatrix<T: WeightNum> {
    c: SquareMatrix<T>,
}

impl<T: WeightNum> WeightMatrix<T> {
    pub fn from_row_vec(n: usize, data: Vec<T>) -> WeightMatrix<T> {
        assert!(n > 0);
        WeightMatrix { c: SquareMatrix::from_row_vec(n, data) }
    }

    pub fn from_fn<F: Fn((usize, usize)) -> T>(n: usize, f: F) -> WeightMatrix<T> {
        assert!(n > 0);
        WeightMatrix { c: SquareMatrix::from_fn(n, f) }
    }

    #[inline(always)]
    pub fn n(&self) -> usize {
        self.c.n()
    }

    #[inline]
    pub fn is_element_zero(&self, pos: (usize, usize)) -> bool {
        self.c[pos].partial_cmp(&T::zero()) == Some(Ordering::Equal)
    }

    // for each row, subtracts the minimum of that row from each other value in the
    // row.
    pub fn sub_min_of_each_row(&mut self) {
        for row in 0..self.n() {
            let min = self.min_of_row(row);
            self.sub_row(row, min);
        }
    }

    /// Return the minimum element of row `row`.
    fn min_of_row(&self, row: usize) -> T {
        let row_slice = self.c.row_slice(row);
        let mut min = row_slice[0];
        for &val in &row_slice[1..] {
            if val < min {
                min = val;
            }
        }
        min
    }

    // Subtract `val` from every element in row `row`.
    fn sub_row(&mut self, row: usize, val: T) {
        self.c.map_row(row, |cur| cur - val);
    }

    // Subtract `val` from every element in column `col`.
    pub fn sub_col(&mut self, col: usize, val: T) {
        self.c.map_col(col, |cur| cur - val);
    }

    // Add `val` to every element in row `row`.
    pub fn add_row(&mut self, row: usize, val: T) {
        self.c.map_row(row, |cur| cur + val);
    }

    /// Find the first uncovered element with value 0 `find_a_zero`
    /// TODO: Move into Coverage as iter_uncovered()
    pub fn find_uncovered_zero(&self, cov: &Coverage) -> Option<(usize, usize)> {
        cov.find_uncovered_col_row(|pos| self.is_element_zero(pos))
    }

    /// Find the smallest uncovered value in the matrix
    pub fn find_uncovered_min(&self, cov: &Coverage) -> Option<T> {
        let mut min = None;
        cov.iter_uncovered_row_col(|pos| {
            let elm = self.c[pos];
            min = Some(match min {
                None => elm,
                Some(m) => {
                    if m < elm {
                        m
                    } else {
                        elm
                    }
                }
            });
        });

        return min;
    }

    pub fn as_slice(&self) -> &[T] {
        self.c.as_slice()
    }
}
