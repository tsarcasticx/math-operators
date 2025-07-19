pub fn factors(n: i128) -> i128 {
    let mut expressions = 1;
    if n <= 0 {
        return 1;
    }
    else {
        for _i in 1..n+1 {
            expressions *= _i;
        }
        return expressions;
    }
}
