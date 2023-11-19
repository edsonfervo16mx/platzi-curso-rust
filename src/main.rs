use regex::Regex;

fn main() {
    let nombre: &str = "Edson";
    let mut edad: u8 = 25;
    edad = edad + 7;
    println!("Hola soy {} y tengo {} anios", nombre, edad);
    hello();
    temperature();
    inputs_by_user();

    ciclos();
}

fn hello() {
    println!("Hello, world!");
}

fn temperature() {
    let temp: i16 = -35;
    println!("Temperature {}", temp);
}

fn inputs_by_user() {
    println!("Por favor ingresa tu nombre");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    //
    println!("Ingresa tu edad");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int: u8 = edad.trim().parse().unwrap();
    println!("Tu nombre es: {} y tu edad es: {}", nombre, edad_int);
    validar_edad(edad_int);
}

fn validar_edad(e: u8) {
    if e >= 18 && e < 21 {
        println!("Tu rango de edad es entre 18 y 21");
    } else if e >= 21 {
        println!("Tu rango de edad es mayor o igual a 21");
    } else {
        println!("Es menor de edad");
    }
}

fn ciclos() {
    let mut i = 0;
    loop {
        if i <= 10 {
            println!("{}", i);
            i = i + 1;
        } else {
            break;
        }
    }
}
