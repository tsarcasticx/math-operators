pub fn fibonaccis(n: i128) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    }
    else {
        return fibonaccis(n - 2) + fibonaccis(n - 1);
    }
}
