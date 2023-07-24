pub fn my_pow(mut x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut n = n as i64;

    if n < 0 {
        n = -1 * n;
        x = 1.0 / x;
    }

    let mut result = 1.0;

    while n != 0 {
        if n % 2 == 1 {
            result = result * x;
            n -= 1;
        }

        x = x * x;
        n = n / 2;
    }

    result
}
