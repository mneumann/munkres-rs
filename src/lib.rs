/// Kuhn-Munkres Algorithm (also called Hungarian algorithm) for solving the
/// Assignment Problem.
///
/// Copyright (c) 2015-2019 by Michael Neumann (mneumann@ntecs.de).
///
/// This code is derived from a port of the Python version found here:
/// https://github.com/bmc/munkres/blob/master/munkres.py
/// which is Copyright (c) 2008 Brian M. Clapper.
use crate::coverage::Coverage;
pub use crate::mark_matrix::{MarkMatrix, MarkMatrixBitArray, MarkMatrixByteArray};
pub use crate::weight_matrix::WeightMatrix;
pub use crate::weight_num::WeightNum;
use ndarray::Array2;

pub type SquareMatrix<T> = Array2<T>;
type MarkMatrixImpl = MarkMatrixByteArray;

mod coverage;
mod mark_matrix;
pub mod weight_matrix;
pub mod weight_num;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub trait Weights {
    type T: WeightNum;
    fn n(&self) -> usize;
    fn element_at(&self, pos: Position) -> Self::T;

    fn sub_min_of_each_row(&mut self);
    fn add_row(&mut self, row: usize, val: Self::T);
    fn sub_column(&mut self, col: usize, val: Self::T);

    #[inline]
    fn is_element_zero(&self, pos: Position) -> bool {
        self.element_at(pos).is_zero()
    }

    fn is_solvable(&self) -> bool;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NoPrimeInRow,
    MatrixNotSolvable,
}

#[derive(Debug, Eq, PartialEq)]
enum Step {
    Step1,
    Step2,
    Step3,
    Step4(Option<usize>),
    Step5(Position),
    Step6,
    Failure(Error),
    Done,
}

/// For each row of the matrix, find the smallest element and
/// subtract it from every element in its row. Go to Step 2.
fn step1<W>(c: &mut W) -> Step
where
    W: Weights,
{
    c.sub_min_of_each_row();
    return Step::Step2;
}

/// Find a zero (Z) in the resulting matrix. If there is no starred
/// zero in its row or column, star Z. Repeat for each element in the
/// matrix. Go to Step 3.
fn step2<W>(c: &W, marks: &mut impl MarkMatrix, cov: &mut Coverage) -> Step
where
    W: Weights,
{
    let n = c.n();

    assert!(marks.n() == n);
    assert!(cov.n() == n);
    debug_assert!(cov.all_uncovered());

    cov.iter_uncovered_row_column_and_cover(|pos| {
        if c.is_element_zero(pos) {
            marks.star(pos);
            true
        } else {
            false
        }
    });

    // clear covers
    cov.clear();

    return Step::Step3;
}

/// Cover each column containing a starred zero. If K columns are
/// covered, the starred zeros describe a complete set of unique
/// assignments. In this case, Go to DONE, otherwise, Go to Step 4.
fn step3<W>(c: &W, marks: &impl MarkMatrix, cov: &mut Coverage) -> Step
where
    W: Weights,
{
    let n = c.n();

    assert!(marks.n() == n);
    assert!(cov.n() == n);

    let mut count: usize = 0;

    marks.each_star(|Position { column, .. }| {
        cov.cover_column(column);
        count += 1;
    });

    if count >= n {
        debug_assert!(count == n);
        Step::Done
    } else {
        Step::Step4(Some(count))
    }
}

/// Find a noncovered zero and prime it. If there is no starred zero
/// in the row containing this primed zero, Go to Step 5. Otherwise,
/// cover this row and uncover the column containing the starred
/// zero. Continue in this manner until there are no uncovered zeros
/// left. Save the smallest uncovered value and Go to Step 6.
fn step4<W>(c: &W, marks: &mut impl MarkMatrix, cov: &mut Coverage) -> Step
where
    W: Weights,
{
    let n = c.n();

    assert!(marks.n() == n);
    assert!(cov.n() == n);

    loop {
        // find uncovered zero element
        match cov.find_uncovered_column_row(|pos| c.is_element_zero(pos)) {
            None => {
                return Step::Step6;
            }
            Some(pos) => {
                marks.prime(pos);
                match marks.find_first_star_in_row(pos.row) {
                    Some(star_col) => {
                        cov.cover_row(pos.row);
                        cov.uncover_column(star_col);
                    }
                    None => {
                        // in Python: self.Z0_r, self.Z0_c
                        return Step::Step5(pos);
                    }
                }
            }
        }
    }
}

/// Construct a series of alternating primed and starred zeros as
/// follows. Let Z0 represent the uncovered primed zero found in Step 4.
/// Let Z1 denote the starred zero in the column of Z0 (if any).
/// Let Z2 denote the primed zero in the row of Z1 (there will always
/// be one). Continue until the series terminates at a primed zero
/// that has no starred zero in its column. Unstar each starred zero
/// of the series, star each primed zero of the series, erase all
/// primes and uncover every line in the matrix. Return to Step 3
fn step5(
    marks: &mut impl MarkMatrix,
    cov: &mut Coverage,
    z0_pos: Position,
    path: &mut Vec<Position>,
) -> Step {
    let n = cov.n();

    assert!(marks.n() == n);

    path.clear();
    path.push(z0_pos);

    let mut prev_col = z0_pos.column;

    loop {
        match marks.find_first_star_in_column(prev_col) {
            Some(row) => {
                path.push(Position {
                    row,
                    column: prev_col,
                });

                if let Some(column) = marks.find_first_prime_in_row(row) {
                    path.push(Position { row, column });
                    prev_col = column;
                } else {
                    // XXX: Can this really happen?
                    return Step::Failure(Error::NoPrimeInRow);
                }
            }
            None => {
                break;
            }
        }
    }

    // convert_path
    for &pos in path.iter() {
        marks.toggle_star(pos);
    }

    cov.clear();
    marks.clear_primes();
    return Step::Step3;
}

/// Add the value found in Step 4 to every element of each covered
/// row, and subtract it from every element of each uncovered column.
/// Return to Step 4 without altering any stars, primes, or covered
/// lines.
fn step6<W>(c: &mut W, cov: &Coverage) -> Step
where
    W: Weights,
{
    let n = c.n();
    assert!(cov.n() == n);

    // Find the smallest uncovered value in the matrix
    let mut min = None;
    cov.iter_uncovered_row_column(|pos| {
        let elm = c.element_at(pos);
        min = Some(match min {
            Some(m) => {
                if m < elm {
                    m
                } else {
                    elm
                }
            }
            None => elm,
        });
    });

    let minval = min.unwrap();
    for row in 0..n {
        if cov.is_row_covered(row) {
            c.add_row(row, minval);
        }
    }
    for column in 0..n {
        if !cov.is_column_covered(column) {
            c.sub_column(column, minval);
        }
    }

    return Step::Step4(None);
}

pub fn solve_assignment<W>(weights: &mut W) -> Result<Vec<Position>, Error>
where
    W: Weights,
{
    solve_assignment_generic::<W, MarkMatrixImpl>(weights)
}

pub fn solve_assignment_generic<W, M>(weights: &mut W) -> Result<Vec<Position>, Error>
where
    W: Weights,
    M: MarkMatrix,
{
    if !weights.is_solvable() {
        return Err(Error::MatrixNotSolvable);
    }

    let n = weights.n();

    let mut marks = M::new(n);
    let mut coverage = Coverage::new(n);
    let mut path = Vec::with_capacity(n);

    let mut step = Step::Step1;
    loop {
        match step {
            Step::Step1 => step = step1(weights),
            Step::Step2 => {
                step = step2(weights, &mut marks, &mut coverage);
            }
            Step::Step3 => {
                step = step3(weights, &marks, &mut coverage);
            }
            Step::Step4(_) => {
                step = step4(weights, &mut marks, &mut coverage);
            }
            Step::Step5(z0_pos) => {
                step = step5(&mut marks, &mut coverage, z0_pos, &mut path);
            }
            Step::Step6 => {
                step = step6(weights, &coverage);
            }
            Step::Failure(err) => {
                return Err(err);
            }
            Step::Done => {
                break;
            }
        }
    }

    // now look for the starred elements
    let mut matching = Vec::with_capacity(n);
    for row in 0..n {
        for column in 0..n {
            let pos = Position { row, column };
            if marks.is_star(pos) {
                matching.push(pos);
            }
        }
    }
    assert!(matching.len() == n);

    return Ok(matching);
}

#[cfg(test)]
fn pos(row: usize, column: usize) -> Position {
    Position { row, column }
}

#[test]
fn test_step1() {
    let c = vec![250, 400, 350, 400, 600, 350, 200, 400, 250];

    let mut weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);

    let next_step = step1(&mut weights);
    assert_eq!(Step::Step2, next_step);

    let exp = &[0, 150, 100, 50, 250, 0, 0, 200, 50];

    assert_eq!(exp, weights.as_slice());
}

#[test]
fn test_step2() {
    let c = vec![0, 150, 100, 50, 250, 0, 0, 200, 50];

    let weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    let next_step = step2(&weights, &mut marks, &mut coverage);
    assert_eq!(Step::Step3, next_step);

    assert_eq!(true, marks.is_star(pos(0, 0)));
    assert_eq!(false, marks.is_star(pos(0, 1)));
    assert_eq!(false, marks.is_star(pos(0, 2)));

    assert_eq!(false, marks.is_star(pos(1, 0)));
    assert_eq!(false, marks.is_star(pos(1, 1)));
    assert_eq!(true, marks.is_star(pos(1, 2)));

    assert_eq!(false, marks.is_star(pos(2, 0)));
    assert_eq!(false, marks.is_star(pos(2, 1)));
    assert_eq!(false, marks.is_star(pos(2, 2)));

    // coverage was cleared
    assert_eq!(false, coverage.is_row_covered(0));
    assert_eq!(false, coverage.is_row_covered(1));
    assert_eq!(false, coverage.is_row_covered(2));
    assert_eq!(false, coverage.is_column_covered(0));
    assert_eq!(false, coverage.is_column_covered(1));
    assert_eq!(false, coverage.is_column_covered(2));
}

#[test]
fn test_step3() {
    let c = vec![0, 150, 100, 50, 250, 0, 0, 200, 50];

    let weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    marks.star(pos(0, 0));
    marks.star(pos(1, 2));

    let next_step = step3(&weights, &marks, &mut coverage);
    assert_eq!(Step::Step4(Some(2)), next_step);

    assert_eq!(true, coverage.is_column_covered(0));
    assert_eq!(false, coverage.is_column_covered(1));
    assert_eq!(true, coverage.is_column_covered(2));

    assert_eq!(false, coverage.is_row_covered(0));
    assert_eq!(false, coverage.is_row_covered(1));
    assert_eq!(false, coverage.is_row_covered(2));
}

#[test]
fn test_step4_case1() {
    let c = vec![0, 150, 100, 50, 250, 0, 0, 200, 50];

    let weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    marks.star(pos(0, 0));
    marks.star(pos(1, 2));
    coverage.cover_column(0);
    coverage.cover_column(2);

    let next_step = step4(&weights, &mut marks, &mut coverage);

    assert_eq!(Step::Step6, next_step);

    // coverage did not change.
    assert_eq!(true, coverage.is_column_covered(0));
    assert_eq!(false, coverage.is_column_covered(1));
    assert_eq!(true, coverage.is_column_covered(2));
    assert_eq!(false, coverage.is_row_covered(0));
    assert_eq!(false, coverage.is_row_covered(1));
    assert_eq!(false, coverage.is_row_covered(2));

    // starring did not change.
    assert_eq!(true, marks.is_star(pos(0, 0)));
    assert_eq!(false, marks.is_star(pos(0, 1)));
    assert_eq!(false, marks.is_star(pos(0, 2)));
    assert_eq!(false, marks.is_star(pos(1, 0)));
    assert_eq!(false, marks.is_star(pos(1, 1)));
    assert_eq!(true, marks.is_star(pos(1, 2)));
    assert_eq!(false, marks.is_star(pos(2, 0)));
    assert_eq!(false, marks.is_star(pos(2, 1)));
    assert_eq!(false, marks.is_star(pos(2, 2)));
}

#[test]
fn test_step6() {
    let c = vec![0, 150, 100, 50, 250, 0, 0, 200, 50];

    let mut weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    marks.star(pos(0, 0));
    marks.star(pos(1, 2));
    coverage.cover_column(0);
    coverage.cover_column(2);

    let next_step = step6(&mut weights, &coverage);

    assert_eq!(Step::Step4(None), next_step);

    let exp = &[0, 0, 100, 50, 100, 0, 0, 50, 50];

    assert_eq!(exp, weights.as_slice());
}

#[test]
fn test_step4_case2() {
    let c = vec![0, 0, 100, 50, 100, 0, 0, 50, 50];

    let weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    marks.star(pos(0, 0));
    marks.star(pos(1, 2));
    coverage.cover_column(0);
    coverage.cover_column(2);

    let next_step = step4(&weights, &mut marks, &mut coverage);

    assert_eq!(Step::Step5(pos(2, 0)), next_step);

    // coverage DID CHANGE!
    assert_eq!(false, coverage.is_column_covered(0));
    assert_eq!(false, coverage.is_column_covered(1));
    assert_eq!(true, coverage.is_column_covered(2));
    assert_eq!(true, coverage.is_row_covered(0));
    assert_eq!(false, coverage.is_row_covered(1));
    assert_eq!(false, coverage.is_row_covered(2));

    // starring DID CHANGE!
    assert_eq!(true, marks.is_star(pos(0, 0)));
    assert_eq!(true, marks.is_prime(pos(0, 1)));
    assert_eq!(true, marks.is_none(pos(0, 2)));
    assert_eq!(true, marks.is_none(pos(1, 0)));
    assert_eq!(true, marks.is_none(pos(1, 1)));
    assert_eq!(true, marks.is_star(pos(1, 2)));
    assert_eq!(true, marks.is_prime(pos(2, 0)));
    assert_eq!(true, marks.is_none(pos(2, 1)));
    assert_eq!(true, marks.is_none(pos(2, 2)));
}

#[test]
fn test_step5() {
    let c = vec![0, 0, 100, 50, 100, 0, 0, 50, 50];

    let weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());

    marks.star(pos(0, 0));
    marks.prime(pos(0, 1));
    marks.star(pos(1, 2));
    marks.prime(pos(2, 0));

    coverage.cover_column(2);
    coverage.cover_row(0);

    let mut path = Vec::new();
    let next_step = step5(&mut marks, &mut coverage, pos(2, 0), &mut path);
    assert_eq!(Step::Step3, next_step);

    // coverage DID CHANGE!
    assert_eq!(false, coverage.is_column_covered(0));
    assert_eq!(false, coverage.is_column_covered(1));
    assert_eq!(false, coverage.is_column_covered(2));
    assert_eq!(false, coverage.is_row_covered(0));
    assert_eq!(false, coverage.is_row_covered(1));
    assert_eq!(false, coverage.is_row_covered(2));

    // starring DID CHANGE!
    assert_eq!(true, marks.is_none(pos(0, 0)));
    assert_eq!(true, marks.is_star(pos(0, 1)));
    assert_eq!(true, marks.is_none(pos(0, 2)));

    assert_eq!(true, marks.is_none(pos(1, 0)));
    assert_eq!(true, marks.is_none(pos(1, 1)));
    assert_eq!(true, marks.is_star(pos(1, 2)));

    assert_eq!(true, marks.is_star(pos(2, 0)));
    assert_eq!(true, marks.is_none(pos(2, 1)));
    assert_eq!(true, marks.is_none(pos(2, 2)));
}

#[test]
fn test_solve() {
    let c = vec![
        250, 400, 350, // row 1
        400, 600, 350, // row 2
        200, 400, 250, // row 3
    ];

    let mut weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(3, c);
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(vec![pos(0, 1), pos(1, 2), pos(2, 0)], matching);
}

#[test]
fn test_solve_equal_rows_stepwise() {
    const N: usize = 2;
    let c = vec![1, 1, 2, 2];

    let mut weights: WeightMatrix<u32> = WeightMatrix::from_row_vec(N, c);

    assert_eq!(1, weights.element_at(pos(0, 0)));
    assert_eq!(1, weights.element_at(pos(0, 1)));
    assert_eq!(2, weights.element_at(pos(1, 0)));
    assert_eq!(2, weights.element_at(pos(1, 1)));

    // step 1

    let next_step = step1(&mut weights);
    assert_eq!(Step::Step2, next_step);
    assert_eq!(&[0, 0, 0, 0], weights.as_slice());

    // step 2

    let mut marks = MarkMatrixImpl::new(weights.n());
    let mut coverage = Coverage::new(weights.n());
    let next_step = step2(&weights, &mut marks, &mut coverage);
    assert_eq!(Step::Step3, next_step);
    assert!(coverage.all_uncovered());

    assert!(marks.is_star(pos(0, 0)));
    assert!(marks.is_star(pos(1, 1)));
    assert!(marks.is_none(pos(0, 1)));
    assert!(marks.is_none(pos(1, 0)));

    // step 3
    let next_step = step3(&weights, &mut marks, &mut coverage);
    assert_eq!(Step::Done, next_step);
}

#[cfg(test)]
fn calc_cost<T>(init_cost: T, c: &[T], matching: &[Position], n: usize) -> T
where
    T: WeightNum,
{
    assert!(c.len() == n * n);
    matching
        .iter()
        .fold(init_cost, |sum, pos| sum + c[pos.row * n + pos.column])
}

#[test]
fn test_solve_equal_rows2() {
    const N: usize = 2;
    let c = vec![1, 1, 2, 2];

    let mut weights: WeightMatrix<u32> = WeightMatrix::from_row_vec(N, c.clone());
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(N, matching.len());
    assert_eq!(3, calc_cost(0, &c[..], &matching[..], N));
}

#[test]
fn test_solve_equal_rows5() {
    const N: usize = 5;
    let c = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let mut weights: WeightMatrix<u32> = WeightMatrix::from_row_vec(N, c.clone());
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(N, matching.len());
    assert_eq!(2, calc_cost(0, &c[..], &matching[..], N));
}

#[test]
fn test_solve_equal_rows5_float() {
    const N: usize = 5;
    let c = vec![
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];

    let mut weights: WeightMatrix<f32> = WeightMatrix::from_row_vec(N, c.clone());
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(N, matching.len());
    assert_eq!(2.0, calc_cost(0.0, &c[..], &matching[..], N));
}

#[test]
fn test_solve_equal_rows5_float2() {
    const N: usize = 5;
    let c = vec![
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    let mut weights: WeightMatrix<f32> = WeightMatrix::from_row_vec(N, c.clone());
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(N, matching.len());
    assert_eq!(3.0, calc_cost(0.0, &c[..], &matching[..], N));
}

#[test]
fn test_solve_random10() {
    const N: usize = 10;
    let c = vec![
        612, 643, 717, 2, 946, 534, 242, 235, 376, 839, 224, 141, 799, 180, 386, 745, 592, 822,
        421, 42, 241, 369, 831, 67, 258, 549, 615, 529, 458, 524, 231, 649, 287, 910, 12, 820, 31,
        92, 217, 555, 912, 81, 568, 241, 292, 653, 417, 652, 630, 788, 32, 822, 788, 166, 122, 690,
        304, 568, 449, 214, 441, 469, 584, 633, 213, 414, 498, 500, 317, 391, 798, 581, 183, 420,
        16, 748, 35, 516, 639, 356, 351, 921, 67, 33, 592, 775, 780, 335, 464, 788, 771, 455, 950,
        25, 22, 576, 969, 122, 86, 74,
    ];

    let mut weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(N, c.clone());
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(N, matching.len());
    assert_eq!(1071, calc_cost(0, &c[..], &matching[..], N));

    let exp = &[
        pos(0, 7),
        pos(1, 9),
        pos(2, 3),
        pos(3, 4),
        pos(4, 1),
        pos(5, 0),
        pos(6, 5),
        pos(7, 6),
        pos(8, 2),
        pos(9, 8),
    ];

    assert_eq!(exp, &matching[..]);
}

#[test]
fn test_disallowed() {
    use std::f32;

    let c = vec![
        250.0,
        400.0,
        350.0,
        400.0,
        600.0,
        f32::INFINITY,
        200.0,
        400.0,
        250.0,
    ];

    let mut weights: WeightMatrix<f32> = WeightMatrix::from_row_vec(3, c);
    let matching = solve_assignment(&mut weights).unwrap();

    assert_eq!(vec![pos(0, 1), pos(1, 0), pos(2, 2)], matching);
}

#[test]
fn test_unsolvable() {
    use std::f32;

    const N: usize = 3;
    let c = vec![
        1.0,
        1.0,
        1.0,
        f32::INFINITY,
        f32::INFINITY,
        f32::INFINITY,
        1.0,
        1.0,
        1.0,
    ];

    let mut weights: WeightMatrix<f32> = WeightMatrix::from_row_vec(N, c.clone());
    let res = solve_assignment(&mut weights);
    assert_eq!(Err(Error::MatrixNotSolvable), res);
}
