fn factorial(x: f32) -> f32 {
    let mut result: f32 = 0.0;
    if x == 0.0 {
        result = 1.0;
    } else if x == 1.0 {
        result = 1.0;
    } else if x == 2.0 {
        result = 2.0;
    } else  {
        for i in 0..x as usize + 1 {
            result += i as f32;
        }
    }
    result
}

fn h(x: &Vec<f32>) -> f32 {
    let result: f32 = x[1] - x[0];
    result
}

fn qx(x_i: &f32, x0: &f32, h: &f32) -> f32 {
    let qx_v = (x_i - x0) / (h);
    qx_v
}

fn mn_qx_j(qx_v: &f32, i: &usize,) -> f32 {
    let mut result: f32 = 1.0;
    if *i == 1 {
        result = *qx_v;
    } else if *i >= 2 {
        for j in 0..*i {
            result *= qx_v - j as f32;
        }
    }
    result
}

fn dy(y: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = y.to_vec();
    for i in 0..result.len() - 1 {
        let ob = y[i + 1] - y[i];
        result[i+1] = ob;
    }
    println!("dy = {:?}", result);
    result
}

fn dy_v(y: &Vec<f32>) -> Vec<f32> {
    let n = y.len();
    let mut result: Vec<f32> = y.to_vec();
    let mut ob = y.to_vec();
    for i in 0..n-1 {
        ob = dy(&ob);
        result[i + 1] = ob[n - 1 - i];
    }
    println!("dy_v = {:?}", result);
    result
}

pub fn nuton(x_i: &f32, x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    let mut result: f32 = 0.0;
    let n = y.len();

    let dys: Vec<f32> = dy_v(y);

    let qx_d = qx(x_i, &x[0], &h(x));

    for i in 1..n {
        let mn_1 = dys[i]/factorial(i as f32);
        let mn_2 = mn_qx_j(&qx_d, &i);

        result += mn_1 * mn_2;

    }
    result += dys[0];
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5.0), 15.0);
    }

    #[test]
    fn test_h() {
        assert_eq!(h(&vec![1.0, 2.0, 3.0, 4.0, 5.0]), 1.0);
    }

    #[test]
    fn test_qx() {
        let mut x_i = 5.0;
        let mut x0 = 0.0;
        let mut h = 1.0;
        assert_eq!(qx(&x_i, &x0, &h), 5.0);
        x_i = 7.0;
        assert_eq!(qx(&x_i, &x0, &h), 7.0);
        x0 = 3.0;
        h = 2.0;
        assert_eq!(qx(&x_i, &x0, &h), 2.0);
    }

}
