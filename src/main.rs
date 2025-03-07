mod nuton;

fn main() {
    let x = vec![15.0, 20.0, 25.0, 30.0, 35.0, 40.0];
    let y = vec![0.2588, 0.3420, 0.4226, 0.5, 0.5736, 0.6428];

    let value_to_interpolate: f32 = 56.0;

    let res_n = nuton::nuton(&value_to_interpolate, &x, &y);
    println!("Nuton interpolation polinom \t- ({};{:?})", value_to_interpolate, res_n);

}