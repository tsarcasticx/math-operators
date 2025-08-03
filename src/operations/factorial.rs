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
