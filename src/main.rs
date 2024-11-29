use meval::Expr;
use std::{io, str::FromStr};

const H: f64 = 0.001;

fn leer_valores() -> String {
    let mut cambio: String = String::new();
    io::stdin()
        .read_line(&mut cambio)
        .expect("Error al leer la linea");
    cambio.trim().to_string()
}

fn determinar_funcion(y: &String, x: &f64) -> f64 {
    let expresion: Expr = Expr::from_str(y).expect("Error al analizar la funcion");
    let funcion = expresion.bind("x").expect("Error al vincular la variable");
    funcion(*x)
}

fn derivada_metodo_numerico(f: &f64, y: &String, h: &f64) -> f64 {
    let f_x_h: f64 = determinar_funcion(&y, h);
    (f_x_h - *f) / H
}

fn metodo_newton(x: &mut f64, f: &f64, f_prima: &f64) -> f64 {
    *x - (f / f_prima)
}

fn main() {
    println!("digite el valor de x");
    let mut x: f64 = leer_valores()
        .parse()
        .expect("Error al convertir a valor numerico");

    println!("digite la funcion. \n- ejemplo : 2*x^3 + 5*x * (2*x - 1)^3 + x^2");
    let y: String = leer_valores();
    let mut h: f64;
    let mut f: f64;
    let mut f_prima: f64;

    for _ in 0..9 {
        h = x + H;
        f = determinar_funcion(&y, &x);
        f_prima = derivada_metodo_numerico(&f, &y, &h);
        x = metodo_newton(&mut x, &f, &f_prima);
    }

    println!("resultado = {x}");
}
