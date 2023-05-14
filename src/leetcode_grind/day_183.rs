// https://leetcode.com/problems/maximize-score-after-n-operations/description/
pub fn max_score(nums: Vec<i32>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    fn backtrack(nums: &Vec<i32>, mask: usize, picked: i32, memo: &mut Vec<i32>) -> i32 {
        if 2 * picked == nums.len() as i32 {
            return 0;
        }

        if memo[mask] != -1 {
            return memo[mask];
        }

        let mut max_score = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (mask >> i) & 1 == 1 || (mask >> j) & 1 == 1 {
                    continue;
                }

                let new_mask = mask | 1 << i | 1 << j;

                let cur_score = (picked + 1) * gcd(nums[i], nums[j]);
                let remaining_score = backtrack(nums, new_mask, picked + 1, memo);

                max_score = max_score.max(cur_score + remaining_score);
            }
        }

        memo[mask] = max_score;
        max_score
    }

    let memo_size = (1 << nums.len() as i32) as usize;
    let mut memo = vec![-1; memo_size];
    backtrack(&nums, 0, 0, &mut memo)
}

// https://leetcode.com/problems/k-th-symbol-in-grammar/description/
// https://leetcode.com/problems/k-th-symbol-in-grammar/solutions/113697/my-3-lines-c-recursive-solution/
pub fn kth_grammar(n: i32, k: i32) -> i32 {
    fn brute_tle(n: i32, k: i32) -> i32 {
        let mut a = "0".to_string();
        for _ in 2..=n {
            let mut next = String::new();
            for ch in a.chars() {
                next.push_str(match ch {
                    '0' => "01",
                    '1' => "10",
                    _ => panic!(),
                });
            }
            a = next
        }
        a.chars().collect::<Vec<_>>()[k as usize - 1] as i32 - '0' as i32
    }
    fn recursive(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if k & 1 == 0 {
            if recursive(n - 1, k / 2) == 0 {
                1
            } else {
                0
            }
        } else {
            if recursive(n - 1, (k + 1) / 2) == 0 {
                0
            } else {
                1
            }
        }
    }

    recursive(n, k)
}

// https://leetcode.com/problems/repeated-substring-pattern/description/
pub fn repeated_substring_pattern(s: String) -> bool {
    if s.len() == 1 {
        return false;
    }
    let s = s.chars().collect::<Vec<_>>();
    let len = s.len();
    for i in 1..=len / 2 {
        let c = s[..i].repeat(len / i);
        if c == s {
            return true;
        }
    }
    false
}

// https://leetcode.com/problems/different-ways-to-add-parentheses/description/
// https://leetcode.com/problems/different-ways-to-add-parentheses/solutions/1294189/easy-solution-faster-than-100-explained-with-diagrams/
pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    fn rec(expression: &[char]) -> Vec<i32> {
        fn compute(x: i32, y: i32, op: char) -> i32 {
            match op {
                '+' => x + y,
                '-' => x - y,
                '*' => x * y,
                _ => panic!(),
            }
        }
        let mut ans = vec![];
        let mut is_num = true;
        for i in 0..expression.len() {
            if !char::is_ascii_digit(&expression[i]) {
                is_num = false;
                let left = rec(&expression[..i]);
                let right = rec(&expression[i + 1..]);

                for &l in &left {
                    for &r in &right {
                        ans.push(compute(l, r, expression[i]));
                    }
                }
            }
        }
        if is_num {
            let num = i32::from_str_radix(expression.into_iter().collect::<String>().as_str(), 10)
                .unwrap();
            ans.push(num);
        }
        ans
    }
    rec(&expression.chars().collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test501() {
        println!("{:?}", diff_ways_to_compute("2*3-4*5".to_string())); // [-34,-14,-10,-10,10]
    }
}
