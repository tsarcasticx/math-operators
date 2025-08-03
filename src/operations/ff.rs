use std::i128;

use super::user;

pub fn factors() -> String {
    let n:i128 = user::user_input_int("Enter the number to be factorialized: ").into();
    let mut expressions: i128 = 1;
    if n <= 0 {
        return 1.to_string();
    }
    else {
        for _i in 1..n+1 {
            expressions *= _i;
        }
        return expressions.to_string();
    }
}
fn fibo(n: i128) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    }
    else {
        return fibo(n - 2) + fibo(n - 1);
    }
}
pub fn fibonaccis() {
    let mut li:Vec<i128> = Vec::new();
    let _value: i128 = user::user_input_int("How many do you want to reveal the pattern?").into();
    println!("\nLoading...");
    for i in 0.._value {
        li.push(fibo(i));
    }
    println!("\nThe pattern is {:?}", li);
    li.pop();
}
