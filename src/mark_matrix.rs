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
        let cell = self.marks.get_mut((pos.row, pos.column)).unwrap();
        if *cell == Mark::Star {
            *cell = Mark::None;
        } else {
            *cell = Mark::Star;
        }
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

    #[inline]
    pub fn each_star<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        for (row, row_data) in self.marks.genrows().into_iter().enumerate() {
            for (column, &cell) in row_data.iter().enumerate() {
                if cell == Mark::Star {
                    f(Position { row, column });
                }
            }
        }
    }

    fn find_first_mark_in_row(&self, row: usize, mark: Mark) -> Option<usize> {
        self.marks
            .row(row)
            .iter()
            .enumerate()
            .find(|(_, &cell)| cell == mark)
            .map(|(column, _)| column)
    }

    fn find_first_mark_in_column(&self, column: usize, mark: Mark) -> Option<usize> {
        self.marks
            .column(column)
            .iter()
            .enumerate()
            .find(|(_, &cell)| cell == mark)
            .map(|(row, _)| row)
    }

    pub fn find_first_star_in_row(&self, row: usize) -> Option<usize> {
        self.find_first_mark_in_row(row, Mark::Star)
    }

    pub fn find_first_prime_in_row(&self, row: usize) -> Option<usize> {
        self.find_first_mark_in_row(row, Mark::Prime)
    }

    pub fn find_first_star_in_column(&self, column: usize) -> Option<usize> {
        self.find_first_mark_in_column(column, Mark::Star)
    }

    pub fn clear_primes(&mut self) {
        for cell in self.marks.iter_mut() {
            if *cell == Mark::Prime {
                *cell = Mark::None
            }
        }
    }
}
