use std::env::{Args, args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().parse().unwrap();
    let second = args.nth(0).unwrap();

    let f_number = first.parse::<f32>().unwrap();
    let s_number = second.parse::<f32>().unwrap();

    let result = operate(operator, f_number, s_number);

    println!("{:?}", output(operator, f_number, s_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }

    // if operator == '+' {
    //     first_number + second_number
    // } else if operator == '-' {
    //     first_number - second_number
    // } else if operator == '/' {
    //     first_number / second_number
    // } else if operator == '*' {
    //     first_number * second_number
    // } else {
    //     0.0
    // }
}

fn output(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
