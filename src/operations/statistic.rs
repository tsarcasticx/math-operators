use crate::operations::user;

fn decil_grouped() -> String{
    let mut _result = String::new();

    //choosing what decil the user wants
    let _choicec_array:Vec<i32> = Vec::from([1,2,3,4,5,6,7,8,9,10]);
    print!("\n{:?}\nChoose the decil above: ", _choicec_array);
    let mut _choose = user::user_input_int();
    //inputing the range from the user
    print!("\nEnter the range \nfor example: the 1-10 value has 10 range: ");
    let _ranges = user::user_input_float();
    print!("\nEnter the total frequency. Must be a number: ");
    let _total_frequency:i32 = user::user_input_int();
    let _i: f64 = (_choose * _total_frequency/10).into();

    print!("\nEnter the cumulative frequency that just one lesser than {} ", _i);
    let _less = user::user_input_float();
    print!("\nEnter the actual frequency that the cumulative frequency is just one greater than or equals to {} ", _i);
    let _freq = user::user_input_float();
    print!("\nLastly, enter the the lower edge value of that actual frequency we've talked about \nfor example: 2-11 value has 1.5 lower edge: ");
    let _tepi_bawah = user::user_input_float();

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
    let mut _choose: i32 = user::user_input_int();
    //inputing the range from the user
    print!("\nEnter the range \nfor example: the 1-10 value has 10 range: ");
    let _ranges = user::user_input_float();
    print!("\nEnter the total frequency. Must be a number: ");
    let _total_frequency:i32 = user::user_input_int();
    let _i: f64 = (_choose * _total_frequency/100).into();

    print!("\nEnter the cumulative frequency that just one lesser than {}", _i);
    let _less = user::user_input_float();
    print!("\nEnter the actual frequency that just one greater than or equals to {}", _i);
    let _freq = user::user_input_float();
    print!("\nLastly, enter the the lower edge value of that actual frequency we've talked about \nfor example: 2-11 value has 1.5 lower edge");
    let _tepi_bawah = user::user_input_float();

    //the actual calculation starts here
    let _j: f64 = (_i - _less).into();
    let _k: f64 = (_j / _freq * _ranges).into();
    let _res: f64 = _k + _tepi_bawah;
    _result.push_str(&_res.to_string());

    return _result;
}


pub fn statistica() {
    print!("\n\n\n What type would you like exactly above? \n\x1B[33m[1] Quartil\n[2] Decil\n[3] Percentil\x1B[37m: ");
            let _sub_choice = user::user_input_string();

            if _sub_choice == "1" {
                println!("We haven't done the operation yet");
            }
            else if _sub_choice == "2" {
                print!("What data would you like to execute enter the number? \n\x1B[33m[1] Single data \n[2] Grouped data\x1B[37m: ");
                let daw = user::user_input_string();
                if daw == "1" {
                    println!("We haven't done the operation yet");
                }
                else if daw == "2" {
                    println!("the result is {}", decil_grouped());
                }
                else {
                    println!("You must enter between 1 or 2");
                }
            }
            else if _sub_choice == "3" {
                print!("What data would you like to execute enter the number? \n\x1B[33m[1] Single data \n[2] Grouped data\x1B[37m: ");
                let daw = user::user_input_string();
                if daw == "1" {
                    println!("We haven't done the operation yet");
                }
                else if daw == "2" {
                    println!("\nthe result is {}\n", percentil_grouped());
                }
                else {
                    println!("You must enter between 1 or 2");
                }
            }
}
