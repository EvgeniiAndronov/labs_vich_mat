use crate::lab1::{ generate_points, find_nearest_square, lagrange_polynomial};
mod lab1;

fn main() {
    let numbers = 200.0;
    let max_points: usize = 7;
    let mut dev = 9999999999.0;
    let mut leng_points = 0;

    let round_val = find_nearest_square(numbers);
    for count_points in 1..=max_points {
        let list_ap = generate_points(round_val, count_points);
        let data = lagrange_polynomial(round_val, list_ap);
        if dev > (data - round_val)  {
            dev = data;
            leng_points = count_points;
        }
    }

    let mut result_vec = generate_points(round_val, leng_points);
    result_vec.push((round_val.powi(2), round_val));

    let result_data = lagrange_polynomial(numbers, result_vec);
    println!("Result: {:.5}", result_data);
}

