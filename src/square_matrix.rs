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

impl<T: Copy> SquareMatrix<T> {
    pub fn from_fn<F: Fn((usize,usize)) -> T>(n: usize, f: F) -> SquareMatrix<T> {
        let data = (0..n*n).map(|i| f((i/n, i%n))).collect();
        SquareMatrix::from_row_vec(n, data)
    }

    pub fn from_row_vec(n: usize, data: Vec<T>) -> SquareMatrix<T> {
        assert!(n > 0); 
        assert!(data.len() == n*n);
        SquareMatrix {n: n, data: data}
    }
    #[inline(always)]
    pub fn n(&self) -> usize { self.n }
    pub fn into_vec(self) -> Vec<T> { self.data }

    #[inline(always)]
    pub fn map_row<F: Fn(T) -> T>(&mut self, row: usize, f: F) {
        for col in 0..self.n {
            let n = f(self[(row, col)]);
            self[(row, col)] = n;
        }
    }

    #[inline(always)]
    pub fn map_col<F: Fn(T) -> T>(&mut self, col: usize, f: F) {
        for row in 0..self.n {
            let n = f(self[(row, col)]);
            self[(row, col)] = n;
        }
    }

}
