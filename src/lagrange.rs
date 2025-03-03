pub fn lagrange(x: f32, x_c: &Vec<f32>, y_c: &Vec<f32>) -> f32 {
    let mut result = 0.0;
    let count_c = x_c.len();

    for i in 0..count_c {
        let mut term = y_c[i];
        for j in 0..count_c {
            if j != i {
                term *= (x - x_c[j]) / (x_c[j] - x_c[i]);
            }
        }
        result += term;
    }
    result = result * (-1.0);
    result
}

pub fn lagrange_output(x_i: &Vec<f32>, x: &Vec<f32>, y: &Vec<f32>) {
    for ob in x_i {
        let y_res = lagrange(*ob, &x, &y);
        println!("Lagrange: {} \t- {}", ob, y_res);
    }
}