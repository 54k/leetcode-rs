// https://leetcode.com/problems/longest-common-subsequence/description/
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let text1 = text1.chars().collect::<Vec<_>>();
    let text2 = text2.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    for i in 1..=text1.len() {
        for j in 1..=text2.len() {
            if text1[i] == text2[j] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[text1.len()][text2.len()]
}

pub fn longest_common_subsequence_backwards(text1: String, text2: String) -> i32 {
    let text1 = text1.chars().collect::<Vec<_>>();
    let text2 = text2.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    for i in (0..text1.len()).rev() {
        for j in (0..text2.len()).rev() {
            if text1[i] == text2[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

pub fn longest_common_subsequence_space_optimized(text1: String, text2: String) -> i32 {
    let mut text1 = text1.chars().collect::<Vec<_>>();
    let mut text2 = text2.chars().collect::<Vec<_>>();
    if text1.len() < text2.len() {
        std::mem::swap(&mut text1, &mut text2);
    }

    let mut previous = vec![0; text1.len() + 1];
    for col in (0..text2.len()).rev() {
        let mut current = vec![0; text1.len() + 1];

        for row in (0..text1.len()).rev() {
            if text1[row] == text2[col] {
                current[row] = 1 + previous[row + 1];
            } else {
                current[row] = previous[row].max(current[row + 1]);
            }
        }
        previous = current;
    }

    previous[0]
}

// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/description/
pub fn number_of_beautiful_integers_tle(low: i32, high: i32, k: i32) -> i32 {
    use std::collections::HashMap;

    fn rec(hi: i32, cur: i32, k: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if hi < cur || cur % k > 0 {
            return 0;
        }
        let mut tmp = cur;
        let mut balance = 0;

        while tmp > 0 {
            let digit = tmp % 10;
            balance += if digit % 2 == 0 { 1 } else { -1 };
            tmp /= 10;
        }

        let mut ans = if balance == 0 { 1 } else { 0 };
        ans += rec(hi, cur + k, k, memo);
        ans
    }

    let mut memo = HashMap::new();
    let hi = rec(high, k, k, &mut memo);
    let lo = rec(low - 1, k, k, &mut memo);
    hi - lo
}

// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/solutions/3931973/digit-dp-c-solution-intuition-explanation-code/
pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
    fn find(
        len: i32,
        tight: bool,
        sum: i32,
        odd: i32,
        is_zero: bool,
        actual_len: i32,
        digits: &Vec<i32>,
        k: i32,
        dp: &mut Vec<Vec<Vec<Vec<Vec<Vec<i32>>>>>>,
    ) -> i32 {
        if len == digits.len() as i32 {
            if (actual_len - odd == odd) && sum == 0 && !is_zero {
                return 1;
            }
            return 0;
        }

        if dp[len as usize][actual_len as usize][tight as usize][sum as usize][odd as usize]
            [is_zero as usize]
            != -1
        {
            return dp[len as usize][actual_len as usize][tight as usize][sum as usize]
                [odd as usize][is_zero as usize];
        }

        let mut limit = 9;
        let mut res = 0;

        if tight {
            limit = digits[len as usize];
        }

        for dig in 0..=limit {
            res += find(
                len + 1,
                tight && (dig == limit),
                (10 * sum + dig) % k,
                odd + (dig % 2),
                is_zero && (dig == 0),
                if is_zero && (dig == 0) {
                    0
                } else {
                    actual_len + 1
                },
                &digits,
                k,
                dp,
            )
        }
        dp[len as usize][actual_len as usize][tight as usize][sum as usize][odd as usize]
            [is_zero as usize] = res;
        res
    }

    fn go(x: i32, k: i32) -> i32 {
        let mut dp = vec![vec![vec![vec![vec![vec![-1; 2]; 11]; 25]; 2]; 11]; 11];
        let mut digits = vec![];
        let mut m = x;
        while m > 0 {
            digits.push(m % 10);
            m /= 10;
        }
        digits.reverse();
        find(0, true, 0, 0, true, 0, &digits, k, &mut dp)
    }

    go(high, k) - go(low - 1, k)
}

#[test]
fn test_number_of_beautiful_integers() {
    let res = number_of_beautiful_integers(5, 5, 2);
    println!("{:?}", res); // 0

    let res = number_of_beautiful_integers(10, 20, 3);
    println!("{:?}", res); // 2
}
