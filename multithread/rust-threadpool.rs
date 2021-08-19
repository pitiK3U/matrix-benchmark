use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Instant;

mod lib;

#[derive(Debug)]
#[repr(transparent)]
struct Matrix<T, const M: usize, const N: usize> ([[T; N]; M]);

fn cross<const M: usize, const N: usize, const K: usize> (lhs: &'static Matrix<isize, M, K>, rhs: &'static Matrix<isize, K, N>) -> Matrix<isize, M, N> {
    let mut new = Matrix([[0isize; N]; M]);
    let threads: usize = std::env::args().nth(1).unwrap_or("1".to_string()).parse::<usize>().unwrap_or(1);
    let pool = lib::ThreadPool::new(threads);


    for row in 0..M {
        for col in 0..N {
            let ptr = AtomicPtr::new(&mut new.0[row][col]);

            pool.execute(move || {
                let mut sum = 0isize;

                for j in 0..K {
                    sum += lhs.0[row][j] * rhs.0[j][col];
                }

                unsafe { *ptr.into_inner() = sum; }
            });
        }
    }

    new
}

fn main() {
    static left: Matrix<isize,50,20> = Matrix([[5; 20]; 50]);
    static right: Matrix<isize,20,15> = Matrix([[2; 15]; 20]);

    let now = Instant::now();

    let result: Matrix<isize, 50_usize, 15_usize> = cross(&left, &right);

    let elapsed = now.elapsed().as_nanos();

    println!("{:?}", result);

    println!("{:#?} ns", elapsed);
}

