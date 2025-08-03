use std::{process::exit};

use crate::operations::{exponential, fibonacci, 
    factorial, statistic, logarithm,user};

mod operations;

fn main() {
    println!("\n\n\nHi! We're doing Math Operators!\n===============================\n\n\n");

    loop {
        let choice = user::user_input_string("What you are going to do?\n\x1B[33m[0] Exit \n[1] Exponential \n[2] Simple Factorial
[3] Fibonacci pattern \n[4] Quartil Decil Percentil\n[5] Logarithm \x1B[37m \nEnter the index: ");
        let _choice:&str = choice.as_str();

        match _choice {
            "0" => {println!("See you next time!"); exit(0);},
            "1" => {let exp = exponential::exponentialis(); println!("\nThe result is {}", exp)},
            "2" => { let _factorial = factorial::factors();
                println!("\nThe result is {}", _factorial);
            },
            "3" => fibonacci::fibonaccis(),
            "4" => statistic::statistica(),
            "5" =>logarithm::logaritma(),
            _ => println!("You must input one of three numbers, not anything else"),
        }
    }
}
