use core::f64;
use std::{io, io::{Write}};

pub fn user_input_string() -> String {
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result = _input.trim();
    return _result.to_string();
}
pub fn user_input_int() -> i32 {
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result: i32 = _input.trim().parse().expect("Undefined user's behaviour");
    return _result;
}
pub fn user_input_float() -> f64 {
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result: f64 = _input.trim().parse().expect("Undefined user's behaviour");
    return _result;
}
