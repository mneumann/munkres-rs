use crate::{Position, SquareMatrix};

#[derive(Debug)]
pub struct MarkMatrix {
    marks: SquareMatrix<Mark>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Mark {
    None,
    Star,
    Prime,
}

impl MarkMatrix {
    pub fn new(n: usize) -> Self {
        Self {
            marks: SquareMatrix::from_shape_fn((n, n), |_| Mark::None),
        }
    }

    #[inline]
    pub fn n(&self) -> usize {
        self.marks.shape()[0]
    }

    #[inline]
    fn get_mark(&self, pos: Position) -> Mark {
        self.marks[(pos.row, pos.column)]
    }

    #[inline]
    fn set_mark(&mut self, pos: Position, mark: Mark) {
        self.marks[(pos.row, pos.column)] = mark;
    }

    pub fn toggle_star(&mut self, pos: Position) {
        if self.is_star(pos) {
            self.unmark(pos);
        } else {
            self.star(pos);
        }
    }

    pub fn unmark(&mut self, pos: Position) {
        self.set_mark(pos, Mark::None);
    }

    pub fn star(&mut self, pos: Position) {
        self.set_mark(pos, Mark::Star);
    }

    pub fn prime(&mut self, pos: Position) {
        self.set_mark(pos, Mark::Prime);
    }

    pub fn is_star(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::Star => true,
            _ => false,
        }
    }

    pub fn is_prime(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::Prime => true,
            _ => false,
        }
    }

    #[cfg(test)]
    pub fn is_none(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::None => true,
            _ => false,
        }
    }

    pub fn each_star<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        let n = self.n();

        for row in 0..n {
            for column in 0..n {
                let pos = Position { row, column };
                if self.is_star(pos) {
                    f(pos);
                }
            }
        }
    }

    pub fn find_first_star_in_row(&self, row: usize) -> Option<usize> {
        for column in 0..self.n() {
            if self.is_star(Position { row, column }) {
                return Some(column);
            }
        }
        return None;
    }

    pub fn find_first_prime_in_row(&self, row: usize) -> Option<usize> {
        for column in 0..self.n() {
            if self.is_prime(Position { row, column }) {
                return Some(column);
            }
        }
        return None;
    }

    pub fn find_first_star_in_column(&self, column: usize) -> Option<usize> {
        for row in 0..self.n() {
            if self.is_star(Position { row, column }) {
                return Some(row);
            }
        }
        return None;
    }

    pub fn clear_primes(&mut self) {
        for row in 0..self.n() {
            for column in 0..self.n() {
                let pos = Position { row, column };
                if self.is_prime(pos) {
                    self.unmark(pos);
                }
            }
        }
    }
}
