pub fn factors(n: i128) -> String {
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
