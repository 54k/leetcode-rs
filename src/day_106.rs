// https://leetcode.com/problems/sum-of-total-strength-of-wizards/description/
// https://leetcode.com/problems/sum-of-total-strength-of-wizards/solutions/2061985/java-c-python-one-pass-solution/
pub fn total_strength(strength: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let strength = strength.into_iter().map(|x| x as i64).collect::<Vec<_>>();
    let N = strength.len();
    let N_i64 = strength.len() as i64;

    let mut ps_l = vec![0; strength.len() + 1];
    let mut pm_l = vec![0; strength.len() + 1];

    for i in 0..N {
        ps_l[i + 1] = (ps_l[i] + strength[i]) % MOD;
        let i_64 = i as i64;
        pm_l[i + 1] = (pm_l[i] + (i_64 + 1) * strength[i]) % MOD;
    }

    let mut ps_r = vec![0; strength.len() + 1];
    let mut pm_r = vec![0; strength.len() + 1];

    for i in (0..N).rev() {
        ps_r[i] = (ps_r[i + 1] + strength[i]) % MOD;
        let i_64 = i as i64;
        pm_r[i] = (pm_r[i + 1] + (N_i64 - i_64) * strength[i]) % MOD;
    }

    let mut stack = vec![];
    let mut ans = 0_i64;

    for right in 0..=N {
        while !stack.is_empty()
            && (right == N || strength[*stack.last().unwrap()] >= strength[right])
        {
            let pivot = stack.pop().unwrap();
            let pivot_i64 = pivot as i64;

            let left_i64 = stack.last().map(|x| *x as i64 + 1).unwrap_or(0);
            let left = left_i64 as usize;

            let right_i64 = right as i64;

            let left_sum = (MOD + pm_l[pivot + 1]
                - pm_l[left]
                - left_i64 * (ps_l[pivot + 1] - ps_l[left]) % MOD)
                % MOD;

            let right_sum = (MOD + pm_r[pivot + 1]
                - pm_r[right]
                - (N_i64 - right_i64) * (ps_r[pivot + 1] - ps_r[right]))
                % MOD;

            let all_sum =
                (left_sum * (right_i64 - pivot_i64) + right_sum * (pivot_i64 - left_i64 + 1)) % MOD;

            ans = (ans + all_sum * strength[pivot]) % MOD;
        }
        stack.push(right);
    }
    ans as i32
}

// https://leetcode.com/problems/edit-distance/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();
    let word1 = word1.chars().collect::<Vec<_>>();
    let word2 = word2.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![10000007; m + 1]; n + 1];
    for i in 1..=n {
        dp[i][0] = i as i32;
    }
    for j in 1..=m {
        dp[0][j] = j as i32;
    }
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 1..=m {
            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test300() {
        println!("{}", total_strength(vec![1, 3, 1, 2])); // 44
        println!("{}", total_strength(vec![5, 4, 6])); // 213
    }

    #[test]
    fn test301() {
        println!("{}", min_distance("horse".to_string(), "ros".to_string())); // 3
    }
}
