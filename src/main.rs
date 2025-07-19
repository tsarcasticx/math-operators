use core::f64;
use std::{i128, io::{self, Write}, process::exit};

use crate::operations::{exponential, fibonacci, factorial, statistic};

mod operations;

fn user_input_string() -> String {
    let mut _s = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _s).unwrap();
    let _stri = _s.trim();
    return _stri.to_string();
}
fn user_input_int() -> i32 {
    let mut _i = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _i).unwrap();
    let _ints: i32 = _i.trim().parse().expect("error");
    return _ints;
}
fn main() {
    println!("\n\n\nHi! We're doing Math Operators!\n===============================\n\n\n");

    loop {
        print!("What you are going to do?\n\x1B[33m[0] Exit \n[1] Exponential \n[2] Simple Factorial
[3] Fibonacci pattern \n[4] Quartil Decil Percentil\x1B[37m \nEnter the index: ");
        let _choice = user_input_string();
    
        if _choice == "0" {
            println!("Thank you for partisipating!");
            exit(0);
        }
        else if _choice == "1" { // then choosing exponential
            print!("\n\nEnter the basis: ");
            let basis: i32 = user_input_int();
            print!("\nEnter the exponent: ");
            let _expo: i32 = user_input_int();
            let _basis: f64 = basis.into();
            println!("\nThe result is {}\n", exponential::exponentialis(_basis, _expo));
        }
        else if _choice == "2" {
            print!("\n\nEnter a number to be factorialized: ");
            let mut _number: i128 = user_input_int().into();
            println!("\nThe result is {}\n", factorial::factors(_number));
        }
        else if _choice == "3" {
            let mut li:Vec<i128> = Vec::new();
            print!("\n\nHow many do you want to reveal the pattern? ");
            let _value: i128 = user_input_int().into();
            println!("\nLoading...");
            for i in 0.._value {
                li.push(fibonacci::fibonaccis(i));
            }
            println!("\nThe pattern is {:?}", li);
            li.pop();
        }
        else if _choice == "4" {
            statistic::statistica();
        }
        else {
                println!("You must input one of three numbers, not anything else");
        }
    }
}
