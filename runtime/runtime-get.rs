use std::time::{Instant};

#[derive(Debug)]
struct Matrix<T, const M: usize, const N: usize> ([[T; N]; M]);

fn cross<const M: usize, const N: usize, const K: usize> (lhs: Matrix<i64, M, K>, rhs: Matrix<i64, K, N>) -> Matrix<i64, M, N> {
    let mut new: Matrix<i64,M,N> = Matrix([[0i64; N]; M]);

    for row in 0..M {
        let new_row = new.0.get_mut(row).unwrap();
        for col in 0..N {
            let mut sum = 0i64;

            let cell = new_row.get_mut(col).unwrap();
            let lhs_row = lhs.0.get(row).unwrap();

            for j in 0..K {
                sum += lhs_row.get(j).unwrap() * rhs.0[j][col];
            }
            *cell = sum;
        }
    }

    new
}

fn main() {
    let left: Matrix<i64,50,20> = Matrix([[5; 20]; 50]);
    let right: Matrix<i64,20,15> = Matrix([[2; 15]; 20]);

    let now = Instant::now();

    let result: Matrix<i64, 50_usize, 15_usize> = cross(left, right);

    let elapsed = now.elapsed().as_nanos();

    println!("{:?}", result);

    println!("{:#?} ns", elapsed);
}

