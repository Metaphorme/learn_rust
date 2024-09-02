fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn main() {
    let c_temperature: f64 = 36.0;
    println!("{c_temperature}°C = {}°F", c_to_f(c_temperature));
}
