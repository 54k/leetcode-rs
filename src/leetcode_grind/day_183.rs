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
