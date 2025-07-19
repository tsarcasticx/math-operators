use core::f64;
use std::{i128, io::{self, Write}, process::exit};

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
fn user_input_float() -> f64 {
    let mut _fl = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut _fl).unwrap();
    let _float: f64 = _fl.trim().parse().expect("error");
    return _float;
}



fn exponential (basis: f64, expo: i32) -> String {
    if expo > 0 {
        let mut b = basis;
        for _i in 1..expo {
            b = b * basis;
        }
        return b.to_string();
    }
    if expo == 0 {
        let b = 1;
        return b.to_string();
    }
    else {
        let mut b:f64 = 1.0;
        let mut c:f64 = 1.0;
        let mut _val = String::new();
        let mut _valu = String::from(" or 1/");
        for _i in expo..0 {
            b = b / basis;
            c = c * basis;
        }
        _val.push_str(&b.to_string());
        _val.push_str(&_valu);
        _val.push_str(&c.to_string());
        return _val;
    }
}
fn factorial(n: i128) -> i128 {
    let mut f = 1;
    for _i in 1..n+1 {
        f *= _i;
    }
    return f;
}
fn fibonacci(n: i128) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    }
    else {
        return fibonacci(n - 2) + fibonacci(n - 1);
    }
}

fn decil_grouped() -> String{
    let mut _result = String::new();

    //choosing what decil the user wants
    let _choicec_array:Vec<i32> = Vec::from([1,2,3,4,5,6,7,8,9,10]);
    print!("\n{:?}\nChoose the decil above: ", _choicec_array);
    let mut _choose = user_input_int();
    //inputing the range from the user
    print!("\nEnter the range \nfor example: the 1-10 value has 10 range: ");
    let _ranges = user_input_float();
    print!("\nEnter the total frequency. Must be a number: ");
    let _total_frequency:i32 = user_input_int();
    let _i: f64 = (_choose * _total_frequency/10).into();

    print!("\nEnter the cumulative frequency that just one lesser than {} ", _i);
    let _less = user_input_float();
    print!("\nEnter the actual frequency that the cumulative frequency is just one greater than or equals to {} ", _i);
    let _freq = user_input_float();
    print!("\nLastly, enter the the lower edge value of that actual frequency we've talked about \nfor example: 2-11 value has 1.5 lower edge: ");
    let _tepi_bawah = user_input_float();

    //the actual calculation starts here
    let _j: f64 = (_i - _less).into();
    let _k: f64 = (_j / _freq * _ranges).into();
    let _res: f64 = _k + _tepi_bawah;
    _result.push_str(&_res.to_string());

    return _result;
}

fn percentil_grouped() -> String{
    let mut _result = String::new();

    //choosing what decil the user wants
    let mut _choicec_array:Vec<i32> = Vec::new();
    for i in 1..101 {
        _choicec_array.push(i);
    }
    print!("\n{:?}\nChoose the percentil above: ", _choicec_array);
    let mut _choose: i32 = user_input_int();
    //inputing the range from the user
    print!("\nEnter the range \nfor example: the 1-10 value has 10 range: ");
    let _ranges = user_input_float();
    print!("\nEnter the total frequency. Must be a number: ");
    let _total_frequency:i32 = user_input_int();
    let _i: f64 = (_choose * _total_frequency/100).into();

    print!("\nEnter the cumulative frequency that just one lesser than {}", _i);
    let _less = user_input_float();
    print!("\nEnter the actual frequency that just one greater than or equals to {}", _i);
    let _freq = user_input_float();
    print!("\nLastly, enter the the lower edge value of that actual frequency we've talked about \nfor example: 2-11 value has 1.5 lower edge");
    let _tepi_bawah = user_input_float();

    //the actual calculation starts here
    let _j: f64 = (_i - _less).into();
    let _k: f64 = (_j / _freq * _ranges).into();
    let _res: f64 = _k + _tepi_bawah;
    _result.push_str(&_res.to_string());

    return _result;
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
            let _basis: i128 = basis.into();
            println!("\nThe result is {}\n", exponential(_basis as f64, _expo));
        }
        else if _choice == "2" {
            print!("\n\nEnter a number to be factorialized: ");
            let mut _number: i128 = user_input_int().into();
            println!("\nThe result is {}\n", factorial(_number));
        }
        else if _choice == "3" {
            let mut li:Vec<i128> = Vec::new();
            print!("\n\nHow many do you want to reveal the pattern? ");
            let _value: i128 = user_input_int().into();
            println!("\nLoading...");
            for i in 0.._value {
                li.push(fibonacci(i));
            }
            println!("\nThe pattern is {:?}", li);
            li.pop();
        }
        else if _choice == "4" {
            print!("\n\n\n What type would you like exactly above? \n\x1B[33m[1] Quartil\n[2] Decil\n[3] Percentil\x1B[37m: ");
            let _sub_choice = user_input_string();

            if _sub_choice == "1" {
                println!("We haven't done the operation yet");
                break;
            }
            else if _sub_choice == "2" {
                print!("What data would you like to execute enter the number? \n\x1B[33m[1] Single data \n[2] Grouped data\x1B[37m: ");
                let daw = user_input_string();
                if daw == "1" {
                    println!("We haven't done the operation yet");
                    break
                }
                else if daw == "2" {
                    println!("the result is {}", decil_grouped());
                }
                else {
                    println!("You must enter between 1 or 2");
                    break;
                }
            }
            else if _sub_choice == "3" {
                print!("What data would you like to execute enter the number? \n\x1B[33m[1] Single data \n[2] Grouped data\x1B[37m: ");
                let daw = user_input_string();
                if daw == "1" {
                    println!("We haven't done the operation yet");
                    break
                }
                else if daw == "2" {
                    println!("\nthe result is {}\n", percentil_grouped());
                }
                else {
                    println!("You must enter between 1 or 2");
                    break;
                }
            }

        }
        else {
                println!("You must input one of three numbers, not anything else");
        }
    }
}
