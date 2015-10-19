use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct SquareMatrix<T> {
    n: usize,
    data: Vec<T>
}

impl<T> Index<(usize, usize)> for SquareMatrix<T> {
    type Output = T;

    /// (row, col)
    #[inline(always)]
    fn index<'a>(&'a self, pos: (usize, usize)) -> &'a T {
        match pos {
            (row, col) => {
                let idx = row * self.n + col;
                &self.data[idx]
            }
        }
    }
}

impl<T> IndexMut<(usize, usize)> for SquareMatrix<T> {
    /// (row, col)
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, pos: (usize, usize)) -> &'a mut T {
        match pos {
            (row, col) => {
                let idx = row * self.n + col;
                &mut self.data[idx]
            }
        }
    }
}

impl<T> SquareMatrix<T> {
    pub fn from_row_vec(n: usize, data: Vec<T>) -> SquareMatrix<T> {
        assert!(data.len() == n*n);
        SquareMatrix {n: n, data: data}
    }
    #[inline(always)]
    pub fn n(&self) -> usize { self.n }
    pub fn into_vec(self) -> Vec<T> { self.data }
}
