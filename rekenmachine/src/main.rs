use core::panic;
use std::io;

fn main() {
    let mut num1_input = String::new();
    println!("Voer je nummer 1 in:");
    io::stdin().read_line(&mut num1_input).expect("Read Error");
    
    let mut num2_input = String::new();
    println!("Voer je nummer 2 in:");
    io::stdin().read_line(&mut num2_input).expect("Read Error");

    let num1: i32 = num1_input.trim().parse().unwrap();
    let num2: i32 = num2_input.trim().parse().unwrap();
    
    let mut operator = String::new();
    println!("+,-,*,/");
    io::stdin().read_line(&mut operator).expect("Read Error");

    let result = match operator.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!()
    };

    println!("Je result is: {result}");

}

//fn berekenen() {

//    let a: f64 = input.trim().parse().expect("Geen geldig getal");
//    let b: f64 = input2.trim().parse().expect("Geen geldig getal");



