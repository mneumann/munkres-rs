use crate::{Position, SquareMatrix, WeightNum, Weights};

#[derive(Debug)]
pub struct WeightMatrix<T: WeightNum> {
    c: SquareMatrix<T>,
}

impl<T: WeightNum> Weights for WeightMatrix<T> {
    type T = T;

    #[inline(always)]
    fn n(&self) -> usize {
        self.c.shape()[0]
    }

    #[inline]
    fn element_at(&self, pos: Position) -> T {
        self.c[(pos.row, pos.column)]
    }

    // for each row, subtracts the minimum of that row from each other value in the
    // row.
    fn sub_min_of_each_row(&mut self) {
        for row in 0..self.n() {
            let min = self.min_of_row(row);
            self.sub_row(row, min);
        }
    }

    // Add `val` to every element in row `row`.
    fn add_row(&mut self, row: usize, val: T) {
        self.c
            .row_mut(row)
            .mapv_inplace(|cur| if cur.is_valid() { cur + val } else { cur });
    }

    // Subtract `val` from every element in column `col`.
    fn sub_column(&mut self, col: usize, val: T) {
        self.c
            .column_mut(col)
            .mapv_inplace(|cur| if cur.is_valid() { cur - val } else { cur });
    }

    fn is_solvable(&self) -> bool {
        for row in 0..self.n() {
            if self.c.row(row).iter().all(|c| !c.is_valid()) {
                return false;
            }
        }
        true
    }
}

impl<T: WeightNum> WeightMatrix<T> {
    pub fn from_row_vec(n: usize, data: Vec<T>) -> WeightMatrix<T> {
        WeightMatrix {
            c: SquareMatrix::from_shape_vec((n, n), data).unwrap(),
        }
    }

    pub fn from_fn<F: Fn((usize, usize)) -> T>(n: usize, f: F) -> WeightMatrix<T> {
        assert!(n > 0);
        WeightMatrix {
            c: SquareMatrix::from_shape_fn((n, n), f),
        }
    }

    /// Return the minimum element of row `row`.
    fn min_of_row(&self, row: usize) -> T {
        let row_slice = self.c.row(row);
        let mut min = row_slice[0];
        for &val in row_slice.iter().skip(1) {
            if val.is_valid() && val < min {
                min = val;
            }
        }
        min
    }

    // Subtract `val` from every element in row `row`.
    fn sub_row(&mut self, row: usize, val: T) {
        self.c
            .row_mut(row)
            .mapv_inplace(|cur| if cur.is_valid() { cur - val } else { cur });
    }

    pub fn as_slice(&self) -> &[T] {
        self.c.as_slice().unwrap()
    }
}

#[test]
fn test_weight_matrix() {
    assert_eq!(0, WeightMatrix::from_row_vec(1, vec![0]).min_of_row(0));
    assert_eq!(1, WeightMatrix::from_row_vec(1, vec![1]).min_of_row(0));
    assert_eq!(
        1,
        WeightMatrix::from_row_vec(2, vec![5, 1, 0, 0]).min_of_row(0)
    );

    let mut mat = WeightMatrix::from_row_vec(2, vec![0, 1, 2, 3]);
    mat.sub_row(1, 1);
    assert_eq!(&[0, 1, 1, 2], mat.as_slice());

    let mut mat = WeightMatrix::from_row_vec(2, vec![5, 3, 2, 3]);
    mat.sub_min_of_each_row();
    assert_eq!(&[2, 0, 0, 1], mat.as_slice());
}
