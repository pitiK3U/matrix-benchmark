use std::time::{Instant};

#[derive(Debug)]
struct Matrix<T, const M: usize, const N: usize> ([[T; N]; M]);

fn cross<const M: usize, const N: usize, const K: usize> (lhs: Matrix<isize, M, K>, rhs: Matrix<isize, K, N>) -> Matrix<isize, M, N> {
    let mut new: Matrix<isize,M,N> = Matrix([[0isize; N]; M]);

    for row in 0..M {
        for col in 0..N {
            let mut sum = 0isize;
            for j in 0..K {
                sum += lhs.0[row][j] * rhs.0[j][col];
            }
            new.0[row][col] = sum;
        }
    }

    new
}

fn main() {
    let left: Matrix<isize,50,20> = Matrix([[5; 20]; 50]);
    let right: Matrix<isize,20,15> = Matrix([[2; 15]; 20]);

    let now = Instant::now();

    let result: Matrix<isize, 50_usize, 15_usize> = cross(left, right);

    let elapsed = now.elapsed().as_nanos();

    println!("{:?}", result);

    println!("{:#?} ns", elapsed);
}

