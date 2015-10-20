use ::square_matrix::SquareMatrix;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mark {
   None,
   Star,
   Prime
}

#[derive(Debug)]
pub struct MarkMatrix {
    marks: SquareMatrix<Mark>
}

impl MarkMatrix {
    pub fn new(n: usize) -> MarkMatrix {
        MarkMatrix {marks: SquareMatrix::from_row_vec(n, (0..n*n).map(|_| Mark::None).collect())}
    }

    pub fn n(&self) -> usize { self.marks.n() }

    pub fn toggle_star(&mut self, pos: (usize, usize)) {
        if self.is_star(pos) {
            self.unmark(pos);
        } else {
            self.star(pos);
        }
    }

    pub fn unmark(&mut self, pos: (usize, usize)) {
       self.marks[pos] = Mark::None; 
    }

    pub fn star(&mut self, pos: (usize, usize)) {
       self.marks[pos] = Mark::Star; 
    }

    pub fn prime(&mut self, pos: (usize, usize)) {
       self.marks[pos] = Mark::Prime; 
    }

    pub fn is_star(&self, pos: (usize, usize)) -> bool {
        match self.marks[pos] {
            Mark::Star => true,
            _          => false 
        }
    }

    pub fn is_prime(&self, pos: (usize, usize)) -> bool {
        match self.marks[pos] {
            Mark::Prime => true,
            _          => false 
        }
    }

    #[cfg(test)]
    pub fn is_none(&self, pos: (usize, usize)) -> bool {
        match self.marks[pos] {
            Mark::None => true,
            _          => false 
        }
    }

    pub fn find_first_star_in_row(&self, row: usize) -> Option<usize> {
        for col in 0..self.n() {
            if self.is_star((row, col)) {
                return Some(col);
            }
        }
        return None;
    }

    pub fn find_first_prime_in_row(&self, row: usize) -> Option<usize> {
        for col in 0..self.n() {
            if self.is_prime((row, col)) {
                return Some(col);
            }
        }
        return None;
    }

    pub fn find_first_star_in_col(&self, col: usize) -> Option<usize> {
        for row in 0..self.n() {
            if self.is_star((row, col)) {
                return Some(row);
            }
        }
        return None;
    }

    pub fn clear_primes(&mut self) {
        for row in 0..self.n() {
            for col in 0..self.n() {
                if self.is_prime((row, col)) {
                    self.unmark((row, col));
                }
            }
        }
    }
}
