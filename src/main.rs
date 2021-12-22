/*
use float_ord::FloatOrd;
use rand::Rng;
use spp_experiments::{sum_of_squares_by_move, sum_of_squares_by_ref, Float};
use std::iter;

/// Create the concrete data-structure of length `n` using FromIterator<V> where V is the element type.
fn create_scrambled_data<V, T>(n: usize) -> T
where
    V: Float<f64>,
    T: iter::FromIterator<V>,
{
    let mut rng = rand::thread_rng();

    (0..n).into_iter().map(|_| V::create(rng.gen())).collect()
}

#[no_mangle]
pub fn process(data: Vec<FloatOrd<f64>>) -> f64 {
    sum_of_squares_by_move(data)
}

fn main() {
    // 1 kB
    let input_size_bytes = 1_000;

    let data_len = input_size_bytes / std::mem::size_of::<f64>();

    let data: Vec<FloatOrd<f64>> = create_scrambled_data(data_len);

    let out = process(data);

    println!("{}", out);
}

*/
fn main() {}
