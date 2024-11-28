fn main() {
    // f(x) = 2x^3, x = 2
    // f'(2) = 24
    let x: f64 = 2.0;
    let h: f64 = 0.001;
    let r1: f64 = 2.0 * x.powi(3);
    let r2: f64 = 2.0 * (2.0 + h).powi(3);
    let r3: f64 = (r2 - r1) / h;

    println!("resultado derivada = {r3}");
}
