// https://leetcode.com/problems/restore-the-array/
pub fn number_of_arrays(s: String, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        if s[i] == '0' {
            continue;
        }
        for j in i..s.len() {
            let num = s[i..=j].iter().collect::<String>().parse::<i64>().unwrap();
            if num > k as i64 {
                break;
            }
            dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
        }
    }
    dp[s.len()] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test456() {
        println!(
            "{}",
            number_of_arrays("1111111111111".to_string(), 1000000000)
        ); // 1
        println!("{}", number_of_arrays("1000".to_string(), 10000)); // 1
        println!("{}", number_of_arrays("1317".to_string(), 2000)); // 8
    }
}
