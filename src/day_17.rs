#[allow(dead_code)]
pub fn number_of_cuts(n: i32) -> i32 {
    match n {
        1 => 0,
        n if n % 2 == 0 => n / 2,
        _ => n,
    }
}

#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut start = 1;
    let mut end = x;
    while start < end {
        let mid = start + (end - start) / 2;
        if mid <= x / mid && (mid + 1) > x / (mid + 1)
        // Found the result
        {
            return mid;
        } else if mid > x / mid {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    start
}

#[cfg(test)]
mod test {
    use crate::day_17::*;

    #[test]
    fn test86() {
        println!("{}", number_of_cuts(4)); // 2
        println!("{}", number_of_cuts(3)); // 3
        println!("{}", number_of_cuts(1)); // 0
    }

    #[test]
    fn test87() {
        println!("{}", my_sqrt(4)); // 2
        println!("{}", my_sqrt(3)); // 1
        println!("{}", my_sqrt(1)); // 1
    }
}
