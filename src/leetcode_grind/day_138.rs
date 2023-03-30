// https://leetcode.com/problems/scramble-string/description/
// https://leetcode.com/problems/scramble-string/editorial/
pub fn is_scramble(s1: String, s2: String) -> bool {
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let n = s1.len();
    let mut dp = vec![vec![vec![false; n]; n]; n + 1];
    for i in 0..n {
        for j in 0..n {
            dp[1][i][j] = s1[i] == s2[j];
        }
    }
    for length in 2..=n {
        for i in 0..n + 1 - length {
            for j in 0..n + 1 - length {
                for new_length in 1..length {
                    let dp1 = dp[new_length][i].clone();
                    let dp2 = dp[length - new_length][i + new_length].clone();
                    dp[length][i][j] |= dp1[j] && dp2[j + new_length];
                    dp[length][i][j] |= dp1[j + length - new_length] && dp2[j];
                }
            }
        }
    }

    dp[n][0][0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test384() {
        println!("{}", is_scramble("great".to_string(), "rgeat".to_string())); // true
        println!("{}", is_scramble("abcde".to_string(), "caebd".to_string())); // false
        println!("{}", is_scramble("a".to_string(), "a".to_string())); // true
    }
}
