pub fn lagrange_polynomial(x: f64, points: Vec<(f64, f64)>) -> f64 {
    let mut result = 0.0;

    for (i, (xi, yi)) in points.iter().enumerate() {
        let mut term = *yi;

        for (j, (xj, _)) in points.iter().enumerate() {
            if i != j {
                term *= (x - xj) / (xi - xj);
            }
        }

        result += term;
    }

    result
}

pub fn generate_points(root: f64, count: usize) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut step = 1;
    let mut direction = 1;

    while points.len() < count {
        let point = root + (step as f64) * (direction as f64);

        points.push((point.powi(2), point));

        direction *= -1;
        if direction == 1 {
            step += 1;
        }
    }

    points
}

pub fn find_nearest_square(value: f64) -> f64 {
    let mut result: f64 = 0.0;
    let mut ii: f64 = 0.0;
    for i in 1..(value / 2.0) as usize {
        if i.pow(2) as f64 <= value {
            ii = i as f64;
        }
    }
    if (ii.powi(2) - value).abs() > ((ii + 1.0).powi(2) - value).abs() {
        result = ii + 1.0
    } else {
        result = ii
    }

    result
}
