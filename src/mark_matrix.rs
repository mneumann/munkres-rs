use crate::{Position, SquareMatrix};
use fixedbitset::FixedBitSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mark {
    None,
    Star,
    Prime,
}

pub trait MarkMatrix {
    fn new(n: usize) -> Self;
    fn n(&self) -> usize;

    fn get_mark(&self, pos: Position) -> Mark;
    fn set_mark(&mut self, pos: Position, mark: Mark);

    fn toggle_star(&mut self, pos: Position) {
        if self.is_star(pos) {
            self.unmark(pos);
        } else {
            self.star(pos);
        }
    }

    fn unmark(&mut self, pos: Position) {
        self.set_mark(pos, Mark::None);
    }

    fn star(&mut self, pos: Position) {
        self.set_mark(pos, Mark::Star);
    }

    fn prime(&mut self, pos: Position) {
        self.set_mark(pos, Mark::Prime);
    }

    fn is_star(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::Star => true,
            _ => false,
        }
    }

    fn is_prime(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::Prime => true,
            _ => false,
        }
    }

    #[cfg(test)]
    fn is_none(&self, pos: Position) -> bool {
        match self.get_mark(pos) {
            Mark::None => true,
            _ => false,
        }
    }

    fn each_star<F>(&self, mut f: F)
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

    fn find_first_star_in_row(&self, row: usize) -> Option<usize> {
        for column in 0..self.n() {
            if self.is_star(Position { row, column }) {
                return Some(column);
            }
        }
        return None;
    }

    fn find_first_prime_in_row(&self, row: usize) -> Option<usize> {
        for column in 0..self.n() {
            if self.is_prime(Position { row, column }) {
                return Some(column);
            }
        }
        return None;
    }

    fn find_first_star_in_column(&self, column: usize) -> Option<usize> {
        for row in 0..self.n() {
            if self.is_star(Position { row, column }) {
                return Some(row);
            }
        }
        return None;
    }

    fn clear_primes(&mut self) {
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

#[derive(Debug)]
pub struct MarkMatrixByteArray {
    marks: SquareMatrix<Mark>,
}

impl MarkMatrix for MarkMatrixByteArray {
    fn new(n: usize) -> Self {
        Self {
            marks: SquareMatrix::from_shape_fn((n, n), |_| Mark::None),
        }
    }

    #[inline]
    fn n(&self) -> usize {
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
}

#[derive(Debug)]
pub struct MarkMatrixBitArray {
    n: usize,
    stars: FixedBitSet,
    primes: FixedBitSet,
}

impl MarkMatrix for MarkMatrixBitArray {
    fn new(n: usize) -> Self {
        Self {
            n,
            stars: FixedBitSet::with_capacity(n),
            primes: FixedBitSet::with_capacity(n),
        }
    }

    #[inline]
    fn n(&self) -> usize {
        self.n
    }

    #[inline]
    fn get_mark(&self, pos: Position) -> Mark {
        let index = pos.row * self.n + pos.column;
        if self.stars.contains(index) {
            Mark::Star
        } else if self.primes.contains(index) {
            Mark::Prime
        } else {
            Mark::None
        }
    }

    #[inline]
    fn set_mark(&mut self, pos: Position, mark: Mark) {
        let index = pos.row * self.n + pos.column;
        let (is_star, is_prime) = match mark {
            Mark::None => (false, false),
            Mark::Star => (true, false),
            Mark::Prime => (false, true),
        };
        self.stars.set(index, is_star);
        self.primes.set(index, is_prime);
    }
}
