use meval::Expr;
use std::{io, str::FromStr};

fn leer_valores() -> String {
    let mut cambio: String = String::new();
    io::stdin()
        .read_line(&mut cambio)
        .expect("Error al leer la linea");
    cambio.trim().to_string()
}

fn main() {
    println!("digite el valor de x");
    let x: i8 = leer_valores()
        .parse()
        .expect("Error al convertir a valor numerico");

    println!("digite y =");
    let y: String = leer_valores();

    let expresion: Expr = Expr::from_str(&y).expect("Error al analizar la funcion");
    let funcion = expresion.bind("x").expect("Error al vincular la variable");
    let resultado: f64 = funcion(x.into());
    println!("resultado = {:.2e}", resultado);
}
