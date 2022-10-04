use std::time::Instant;

#[derive(Debug)]
struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

fn cross<const M: usize, const N: usize, const K: usize>(
    lhs: Matrix<i64, M, K>,
    rhs: Matrix<i64, K, N>,
) -> Matrix<i64, M, N> {
    let mut new: Matrix<i64, M, N> = Matrix([[0i64; N]; M]);

    new.0
        .iter_mut()
        .zip(lhs.0.iter())
        .for_each(|(new_row, lhs_row)| {
            lhs_row
                .iter()
                .zip(rhs.0.iter())
                .for_each(|(lhs_cell, rhs_row)| {
                    new_row
                        .iter_mut()
                        .zip(rhs_row.iter())
                        .for_each(|(new_cell, rhs_cell)| *new_cell += lhs_cell * rhs_cell)
                })
        });

    new
}

fn main() {
    let left: Matrix<i64, 50, 20> = Matrix([[5; 20]; 50]);
    let right: Matrix<i64, 20, 15> = Matrix([[2; 15]; 20]);

    let now = Instant::now();

    let result: Matrix<i64, 50_usize, 15_usize> = cross(left, right);

    let elapsed = now.elapsed().as_nanos();

    println!("{:?}", result);

    println!("{:#?} ns", elapsed);
}
