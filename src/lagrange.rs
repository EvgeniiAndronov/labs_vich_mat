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
