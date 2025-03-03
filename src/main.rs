use std::time::{Instant};

mod lagrange;
mod nuton;

fn main() {
    let x = vec![0.0, 2.0, 4.0, 6.0];
    let y = vec![-5.0, -3.0, 31.0, 145.0];

    let value_to_interpolate: f32 = 1.0;

    let res_n = nuton::nuton(&value_to_interpolate, &x, &y);
    println!("Nuton interpolation polinom \t- ({};{:?})", value_to_interpolate, res_n);

    let res_l = lagrange::lagrange(value_to_interpolate, &x, &y);
    println!("Lagrange polinom interpolation \t- ({};{:?})", value_to_interpolate, res_l);
}