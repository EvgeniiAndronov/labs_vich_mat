// Вычисление квадратного корня через интерполяцию Лагранжа
pub fn lagrange_sqrt(n: f64, points: usize) -> f64 {
    // Находим ближайший квадрат
    let k = n.sqrt().floor() as i32;
    let mut nodes = Vec::new();

    // Создаем узлы интерполяции вокруг целевого значения
    for i in (k - points as i32 + 1)..=k {
        let x = i as f64;
        nodes.push((x * x, x)); // (x², x)
    }

    // Интерполяционный полином Лагранжа
    let mut result = 0.0;
    for (i, (xi, yi)) in nodes.iter().enumerate() {
        let mut term = *yi;
        for (j, (xj, _)) in nodes.iter().enumerate() {
            if i != j {
                term *= (n - xj) / (xi - xj);
            }
        }
        result += term;
    }
    result
}

// Поиск оптимального количества узлов
pub fn find_optimal_sqrt(n: f64, max_points: usize) -> (f64, usize) {
    let true_sqrt = n.sqrt();
    let mut best_error = f64::MAX;
    let mut best_guess = 0.0;
    let mut best_points = 0;

    for points in 2..=max_points {
        let approx = lagrange_sqrt(n, points);
        let error = (approx - true_sqrt).abs();

        if error < best_error {
            best_error = error;
            best_guess = approx;
            best_points = points;
        }
    }

    (best_guess, best_points)
}

