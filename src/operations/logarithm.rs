use crate::user;

pub fn logaritma() {
    print!("\nEnter the base: ");
    let _base = user::user_input_float();
    print!("\nEnter the exponent: ");
    let _exponent = user::user_input_int();
}
