use crate::lab1::find_optimal_sqrt;

// mod nuton;
// mod lagrange;
mod lab1;

// fn main() {
//     let x_i: f32 = 1.0;
//     let x: Vec<f32> = vec![0.0, 2.0, 4.0, 6.0];
//     let y: Vec<f32> = vec![-5.0, -3.0, 31.0, 145.0];
//
//     let value_to_interpolate: f32 = x_i;
//
//     let res_n = nuton::nuton(&value_to_interpolate, &x, &y);
//     let res_l = lagrange::lagrange(value_to_interpolate, &x, &y);
//     println!("Nuton interpolation polinom \t- ({}; {:?})\nLagrange interpolation polinom \t- ({}; {:?})", value_to_interpolate, res_n, value_to_interpolate, res_l);
//
//     lab1::st();
// }

fn main() {
    let numbers = [16.0, 25.0, 30.0, 100.0, 150.0, 20.0];
    let max_points = 10;

    for &n in &numbers {
        let (approx, points) = find_optimal_sqrt(n, max_points);
        let real = n.sqrt();
        let error = (approx - real).abs();

        println!("Число: {:5.1} | Корень: {:5.3} | Приближение: {:5.3} | Узлы: {} | Погрешность: {:.3e}",
                 n, real, approx, points, error);
    }
}