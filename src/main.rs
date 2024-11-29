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

fn determinar_funcion(y: &String, x: f64) -> f64 {
    let expresion: Expr = Expr::from_str(y).expect("Error al analizar la funcion");
    let funcion = expresion.bind("x").expect("Error al vincular la variable");
    let resultado: f64 = funcion(x);
    resultado
}

fn derivada_metodo_numerico(resultado_funcion: &f64, y: &String, x: &f64) -> f64 {
    let f: f64 = determinar_funcion(&y, *x + H);
    let resultado = (f - resultado_funcion) / H;
    resultado
}

fn metodo_newton(x: &f64, resultado_funcion: &f64, resultado_derivada: f64) -> f64 {
    x - (resultado_funcion / resultado_derivada)
}

fn main() {
    println!("digite el valor de x");
    let x: f64 = leer_valores()
        .parse()
        .expect("Error al convertir a valor numerico");

    println!("digite la funcion");
    let y: String = leer_valores();

    let resultado_funcion: f64 = determinar_funcion(&y, x);
    let derivada: f64 = derivada_metodo_numerico(&resultado_funcion, &y, &x);
    let resultado: f64 = metodo_newton(&x, &resultado_funcion, derivada);
    println!("resultado = {resultado}");
}
