use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct SquareMatrix<T> {
    n: usize,
    data: Box<[T]>,
}

impl<T> Index<(usize, usize)> for SquareMatrix<T> {
    type Output = T;

    /// (row, col)
    #[inline(always)]
    fn index<'a>(&'a self, pos: (usize, usize)) -> &'a T {
        let (row, col) = pos;
        let idx = row * self.n + col;
        &self.data[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for SquareMatrix<T> {
    /// (row, col)
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, pos: (usize, usize)) -> &'a mut T {
        let (row, col) = pos;
        let idx = row * self.n + col;
        &mut self.data[idx]
    }
}

impl<T: Copy> SquareMatrix<T> {
    pub fn from_fn<F: Fn((usize, usize)) -> T>(n: usize, f: F) -> SquareMatrix<T> {
        let data = (0..n * n).map(|i| f((i / n, i % n))).collect();
        SquareMatrix::from_row_vec(n, data)
    }

    pub fn from_row_vec(n: usize, data: Vec<T>) -> SquareMatrix<T> {
        assert!(n > 0);
        assert!(data.len() == n * n);
        SquareMatrix {
            n: n,
            data: data.into_boxed_slice(),
        }
    }

    #[inline(always)]
    pub fn n(&self) -> usize {
        self.n
    }

    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        &self.data[..]
    }

    #[inline]
    pub fn row_slice(&self, row: usize) -> &[T] {
        assert!(row < self.n);
        &self.data[row * self.n..(row + 1) * self.n]
    }

    #[inline]
    pub fn row_slice_mut(&mut self, row: usize) -> &mut [T] {
        assert!(row < self.n);
        &mut self.data[row * self.n..(row + 1) * self.n]
    }

    #[inline]
    pub fn map_row<F: Fn(T) -> T>(&mut self, row: usize, f: F) {
        let n = self.n;
        assert!(row < n);

        let mut row_slice = self.row_slice_mut(row);
        debug_assert!(row_slice.len() == n);

        for elm in row_slice.iter_mut() {
            *elm = f(*elm);
        }
    }

    #[inline]
    pub fn map_col<F: Fn(T) -> T>(&mut self, col: usize, f: F) {
        for row in 0..self.n {
            let mut elm = &mut self[(row, col)];
            *elm = f(*elm);
        }
    }

}

#[test]
fn test_square_matrix() {
    let mat = SquareMatrix::from_row_vec(2, vec![1, 2, 3, 4]);
    assert_eq!(2, mat.n());

    assert_eq!(1, mat[(0,0)]);
    assert_eq!(2, mat[(0,1)]);
    assert_eq!(3, mat[(1,0)]);
    assert_eq!(4, mat[(1,1)]);

    assert_eq!(&[1,2,3,4], mat.as_slice());
    assert_eq!(&[1,2], mat.row_slice(0));
    assert_eq!(&[3,4], mat.row_slice(1));

    let mut mat = mat;

    mat.map_row(0, |i| i + 5);
    assert_eq!(&[6,7,3,4], mat.as_slice());

    mat.map_col(1, |i| i + 2);
    assert_eq!(&[6,9,3,6], mat.as_slice());
    mat.map_col(0, |i| i + 3);
    assert_eq!(&[9,9,6,6], mat.as_slice());


    let mat = SquareMatrix::from_fn(3, |(row, _col)| row);
    assert_eq!(&[0,0,0,1,1,1,2,2,2], mat.as_slice());

    let mat = SquareMatrix::from_fn(3, |(_row, col)| col);
    assert_eq!(&[0,1,2,0,1,2,0,1,2], mat.as_slice());

    let mat = SquareMatrix::from_fn(3, |(row, col)| row + col);
    assert_eq!(&[0,1,2,1,2,3,2,3,4], mat.as_slice());
}
