use regex::Regex;

fn main() {
    println!("Hola");
    // regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_multi = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let re_resta = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    // input
    println!("Por favor introduce tu expresion:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    // operations
    //Multiplicacion
    loop {
        let caps = re_multi.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let multi = left_value * right_value;
        expression = expression.replace(cap_expression, &multi.to_string());
    }
    //Division
    loop {
        let caps = re_div.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let div = left_value / right_value;
        expression = expression.replace(cap_expression, &div.to_string());
    }
    //Suma
    loop {
        let caps = re_add.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value + right_value;
        expression = expression.replace(cap_expression, &addition.to_string());
    }
    //Resta
    loop {
        let caps = re_resta.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value - right_value;
        expression = expression.replace(cap_expression, &addition.to_string());
    }
    // output
    println!("Tu resultado es: {}", expression);
}
