use regex::Regex;

fn main() {
    println!("Hola");
    // regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    // input
    println!("Por favor introduce tu expresion:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    // operations
    let caps = re_add.captures(&expression.as_str()).unwrap();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    // println!("{:?}", caps);
    println!("{}", left_value);
    println!("{}", right_value);

    let result = left_value + right_value;
    // output
    println!("Tu expresion es: {}", expression);
    println!("Tu resultado es: {}", result);
}
