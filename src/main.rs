use std::io;

fn leer_valores() -> String {
    let mut cambio: String = String::new();
    io::stdin()
        .read_line(&mut cambio)
        .expect("Error al leer la linea");
    cambio
}

fn main() {
    println!("digite el valor de x");
    let x: String = leer_valores();
    // let x: i16 = x.trim().parse().expect("Error");
    println!("valor de x = {x}");

    println!("digite la funcion");
    let mut funcion: String = leer_valores();
    funcion = funcion.replace('x', &x);
    println!("funcion = {funcion}");
    let resultado: i8 = funcion.trim().parse().expect("Error");

    println!("y = {resultado}");

    /*
    // f(x) = 2x^3, x = 2
    //f'(2) = 24
    let x: f64 = 2.0;
    let h: f64 = 0.001;
    let r1: f64 = 2.0 * x.powi(3);
    let r2: f64 = 2.0 * (2.0 + h).powi(3);
    let r3: f64 = (r2 - r1) / h;

    println!("resultado derivada = {r3}");
    */
}
