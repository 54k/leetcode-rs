fn factorial(n: i32) -> i64 {
    let mut ans = 1;
    for i in 1..=n as i64 {
        ans *= i;
    }
    ans
}

fn fac_odd(n: i32) -> i64 {
    let mut ans = 1;
    for i in 1..=n as i64 {
        if i % 2 > 0 {
            println!("{}", i);
            ans *= i;
        }
    }
    ans
}

fn c(n: i32, k: i32) -> i64 {
    let b = factorial(k) * factorial(n - k);
    factorial(n) / b
}

fn a(n: i32, k: i32) -> i64 {
    factorial(n) / factorial(n - k)
}

fn p(n: i32, m: i32, k: i32) -> i64 {
    if n == 0 && k == 0 {
        1
    } else if k == 0 {
        0
    } else if k <= n {
        p(n, m, k - 1) + p(n - k, m - 1, k)
    } else {
        p(n, m, n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_c1() {
        println!("{}", (c(10, 5) * c(11, 5)));
    }

    #[test]
    fn test_c2() {
        println!("{}", factorial(10));
    }

    #[test]
    fn test_c3() {
        println!("{}", fac_odd(19));
    }

    #[test]
    fn test_c4() {
        println!("{}", c(20, 10) - 1);
    }

    #[test]
    fn test_c5() {
        println!("{}", c(3 + 2 - 1, 3 - 1));
    }

    #[test]
    fn test_c6() {
        println!("{}", c(8, 5));
    }

    #[test]
    fn test_c7() {
        println!("{}", c(6 + 4 - 1, 6 - 1));
    }

    #[test]
    fn test_c8() {
        println!("{}", factorial(9) / 2);
        println!("{}", factorial(10) / 10 / 2);
        println!("{}", factorial(4) / 4 / 2);
        println!("{}", factorial(3));
    }

    #[test]
    fn test_c9() {
        let n = 8;
        let k = 3;

        println!("{}", c(n - k - 1, k - 1) + c(n - k, k));
    }

    #[test]
    fn test_c10() {
        // Искомый коэффициент равен числу решений уравнения p + q + r = 100 в целых неотрицательных числах.
        let mut ans = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                for k in 0..=100 {
                    if i + j + k == 100 {
                        ans += 1;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
