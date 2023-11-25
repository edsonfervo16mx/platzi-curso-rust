fn main() {
    let mut names: Vec<String> = Vec::new();

    for _ in 0..5 {
        println!("Por favor introduce un nombre: ");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        names.push(name);
    }
    // println!("Por favor introduce un nombre: ");
    // let mut name = String::new();
    // std::io::stdin().read_line(&mut name).unwrap();

    // names.push(name);

    println!("Los nombres introducidos son: {:?}", names);

    for nombre in names {
        println!("{}", nombre);
    }
}
