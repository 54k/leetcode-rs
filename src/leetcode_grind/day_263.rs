// https://leetcode.com/problems/find-the-value-of-the-partition/
pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut ans = i32::MAX;
    for i in (1..nums.len()).rev() {
        ans = ans.min((nums[i - 1] - nums[i]).abs());
    }
    ans
}

// https://leetcode.com/problems/special-permutations/
pub fn special_perm(nums: Vec<i32>) -> i32 {
    pub fn special_perm_tle(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let mut ans = 0;
        fn permute(nums: &Vec<i32>, cur: &mut Vec<i32>, mask: i32, ans: &mut i64) {
            if cur.len() == nums.len() {
                println!("{:?}", cur);
                let mut special = true;

                for i in 0..cur.len() - 1 {
                    if (cur[i] % cur[i + 1]) != 0 && (cur[i + 1] % cur[i]) != 0 {
                        special = false;
                    }
                }

                if special {
                    *ans = ((*ans % MOD) + (1 % MOD)) % MOD;
                }
                return;
            }

            for i in 0..nums.len() {
                if mask & (1 << i) == 0 {
                    cur.push(nums[i]);
                    permute(nums, cur, mask | (1 << i), ans);
                    cur.pop();
                }
            }
        }
        permute(&nums, &mut vec![], 0, &mut ans);
        ans as i32
    }

    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let mut memo = vec![vec![-1; 16]; (1 << 14) + 1];
        fn rec(nums: &Vec<i32>, cnt: i32, i: i32, mask: i32, memo: &mut Vec<Vec<i64>>) -> i64 {
            if cnt == nums.len() as i32 {
                return 1;
            }

            if memo[mask as usize][(i + 1) as usize] != -1 {
                return memo[mask as usize][(i + 1) as usize];
            }

            let mut res = 0;
            for j in 0..nums.len() {
                if i == -1
                    || ((mask & (1 << j)) == 0
                        && i as usize != j
                        && ((nums[i as usize] % nums[j] == 0) || (nums[j] % nums[i as usize] == 0)))
                {
                    res = (res + rec(nums, cnt + 1, j as i32, mask | (1 << j), memo)) % MOD;
                }
            }
            memo[mask as usize][(i + 1) as usize] = res;
            res
        }

        rec(&nums, 0, -1, 0, &mut memo) as i32
    }

    special_perm(nums)
}

// https://leetcode.com/problems/get-maximum-in-generated-array/description/
pub fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut nums = vec![0; n as usize + 1];
    let mut ans = 0;
    nums[1] = 1;
    for i in 2..nums.len() {
        if i % 2 == 0 {
            nums[i] = nums[i / 2];
        } else {
            nums[i] = nums[i / 2] + nums[i / 2 + 1];
        }
        ans = ans.max(nums[i]);
    }
    ans
}

// https://leetcode.com/problems/word-break-ii/description/
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;
    let dict = word_dict.into_iter().collect::<HashSet<String>>();
    let mut ans = vec![];
    fn dfs(s: String, cur: &mut Vec<String>, dict: &HashSet<String>, ans: &mut Vec<String>) {
        if s.is_empty() {
            ans.push(cur.clone().join(" "));
            return;
        }

        for w in dict {
            if let Some(0) = s.find(w) {
                cur.push(w.clone());
                dfs(s[w.len()..].to_string(), cur, dict, ans);
                cur.pop();
            }
        }
    }
    dfs(s, &mut vec![], &dict, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cnt_perms() {
        println!("{}", special_perm(vec![2, 3, 6]));
    }

    #[test]
    fn test_word_break() {
        println!(
            "{:?}",
            word_break(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "and".to_string(),
                    "sand".to_string(),
                    "dog".to_string(),
                ],
            )
        );
    }
}
