fn main() {
    println!("Ingresar un numero:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Ingresar el numero de ceros");
    let mut ceros = String::new();
    std::io::stdin().read_line(&mut ceros).unwrap();

    generate_folio(
        input.trim().parse::<i32>().unwrap(),
        ceros.trim().parse::<i32>().unwrap(),
    );

    test_format();
}

fn generate_folio(n: i32, ceros: i32) {
    let formatted_folio = format!("{:0width$}", n, width = ceros as usize);

    println!("El folio es: {}", formatted_folio);
}

fn test_format() {
    let string = format!("{:0width$}", 1, width = 5);
    // 00001
    println!("{}", string);
}
