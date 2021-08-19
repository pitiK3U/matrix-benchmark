use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;


#[derive(Debug)]
struct Matrix<T, const M: usize, const N: usize> ([[T; N]; M]);

fn cross<const M: usize, const N: usize, const K: usize> (lhs: &'static Matrix<isize, M, K>, rhs: &'static Matrix<isize, K, N>) -> Matrix<isize, M, N> {
    let new = Arc::new(Mutex::new(Matrix([[0isize; N]; M])));
    let mut handles = vec![];

    for row in 0..M {
        for col in 0..N {
            let clonned = Arc::clone(&new);
            let thread = thread::spawn(move || {
                let mut sum = 0isize;

                for j in 0..K {
                    sum += lhs.0[row][j] * rhs.0[j][col];
                }

                let mut matrix = clonned.lock().unwrap();
                matrix.0[row][col] = sum;
            });

            handles.push(thread);
        }
    }

    for handle in handles {
        handle.join().unwrap()
    }

    // println!("Strong count: {}, weak count: {}", Arc::strong_count(&new), Arc::weak_count(&new));

    // while Arc::strong_count(&new) > 1 {}

    Arc::try_unwrap(new).unwrap().into_inner().expect("Mutex cannot be locked")
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

