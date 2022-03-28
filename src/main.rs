use std::env::{args, Args};

fn main() {
    let mut list = args();

    let first = list.nth(1).unwrap();
    let operator = list.nth(0).unwrap().chars().next().unwrap();
    let second = list.nth(0).unwrap();

    let num1: f32 = first.parse().unwrap();
    let num2: f32 = second.parse().unwrap();
    let result = operate(operator, num1, num2);

    let s = format!("{} {} {} = {}", first, operator, second, result);

    println!("{}", s);
}

fn operate(operator: char, num1: f32, num2: f32) -> f32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        _ => 0.0,
    }
}
