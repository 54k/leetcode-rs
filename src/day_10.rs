#[allow(dead_code)]
pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    for i in 1..=n {
        let mut min = i32::MAX;
        let mut j = 1;
        while i - j * j >= 0 {
            min = std::cmp::min(min, dp[(i - j * j) as usize] + 1);
            j += 1
        }
        dp[i as usize] = min;
    }
    dp[n as usize]
}

#[cfg(test)]
mod test {
    use crate::day_10::*;

    #[test]
    fn test59() {
        println!("{}", num_squares(12));
    }
}
