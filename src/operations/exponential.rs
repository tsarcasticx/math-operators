use super::user;

pub fn exponentialis () -> String {
// basis: f64, expo: i32
    let basis = user::user_input_float("Enter the basis: ");
    let expo = user::user_input_int("Enter the exponential: ");
    if expo > 0 {
        let mut b = basis;
        for _i in 1..expo {
            b *= basis;
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
            b /= basis;
            c *= basis;
        }
        _val.push_str(&b.to_string());
        _val.push_str(&_valu);
        _val.push_str(&c.to_string());
        return _val;
    }
}
