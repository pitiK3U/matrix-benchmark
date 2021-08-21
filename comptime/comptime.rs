use std::time::{Instant};

#[derive(Debug)]
struct Matrix<T, const M: usize, const N: usize> ([[T; N]; M]);

const fn cross<const M: usize, const N: usize, const K: usize> (lhs: Matrix<i64, M, K>, rhs: Matrix<i64, K, N>) -> Matrix<i64, M, N> {
    let mut new: Matrix<i64,M,N> = Matrix([[0i64; N]; M]);

    let mut row = 0usize;
    while row < M {
    // for row in 0..M {
        let mut col = 0usize;
        while col < N {
        //for col in 0..N {
            let mut sum = 0i64;
            let mut j   = 0usize;
            while j < K {
            //for j in 0..K {
                sum += lhs.0[row][j] * rhs.0[j][col];
                j += 1;
            }
            new.0[row][col] = sum;

            col += 1;
        }

        row += 1;
    }

    new
}

fn main() {
    const left: Matrix<i64,50,20> = Matrix([[5; 20]; 50]);
    const right: Matrix<i64,20,15> = Matrix([[2; 15]; 20]);

    let now = Instant::now();

    const result: Matrix<i64, 50_usize, 15_usize> = cross(left, right);

    let elapsed = now.elapsed().as_nanos();

    println!("{:?}", result);

    println!("{:#?} ns", elapsed);
}

