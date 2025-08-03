use core::f64;
use std::{io::{self, Write}, process::exit};

pub fn user_input_string(msg:&str) -> String {
    print!("\n{}",msg);
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result = _input.trim();
    return _result.to_string();
}
fn check_int(msg:&str) -> Result<i32,&str> {
    print!("\n{}",msg);
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result: Result<i32, _> = _input.trim().parse();
    let positive:i32;
    let negative:&str;
    match _result {
        Ok(t) => {positive = t; return Ok(positive);},
        Err(_e) => {negative = "You must enter the integer, not any"; return Err(negative);}
    };
}
fn check_float(msg:&str) -> Result<f64,&str> {
    print!("\n{}",msg);
    let mut _input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _input).unwrap();
    
    let _result: Result<f64, _> = _input.trim().parse();
    let positive:f64;
    let negative:&str;
    match _result {
        Ok(t) => {positive = t; return Ok(positive);},
        Err(_e) => {negative = "You must enter the number, not any"; return Err(negative);}
    }
}

pub fn user_input_int(msg:&str) -> i32 {
    let _int = check_int(msg);
    if let Err(_e) = _int {
        eprintln!("{_e}");
        exit(1)
    }
    return _int.unwrap();
}
pub fn user_input_float(msg:&str) -> f64 {
    let _float = check_float(msg);
    if let Err(_e) = _float {
        eprintln!("{_e}");
        exit(1)
    }
    return _float.unwrap();
}
