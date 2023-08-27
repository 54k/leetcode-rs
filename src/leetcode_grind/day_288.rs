// https://leetcode.com/problems/frog-jump/description/
pub fn can_cross(stones: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut mark = HashMap::new();
    for (i, &s) in stones.iter().enumerate() {
        mark.insert(s, i as i32);
    }
    let mut dp = vec![vec![-1; 2001]; 2001];

    fn solve(
        dp: &mut Vec<Vec<i32>>,
        stones: &Vec<i32>,
        mark: &mut HashMap<i32, i32>,
        index: i32,
        prev_jump: i32,
    ) -> bool {
        if index == stones.len() as i32 - 1 {
            return true;
        }

        if dp[index as usize][prev_jump as usize] != -1 {
            return dp[index as usize][prev_jump as usize] == 1;
        }

        let mut ans = false;

        for next_jump in prev_jump - 1..=prev_jump + 1 {
            if next_jump > 0 && mark.contains_key(&(stones[index as usize] + next_jump)) {
                ans = ans
                    || solve(
                        dp,
                        stones,
                        mark,
                        mark[&(stones[index as usize] + next_jump)],
                        next_jump,
                    );
            }
        }

        dp[index as usize][prev_jump as usize] = if ans { 1 } else { 0 };
        ans
    }
    solve(&mut dp, &stones, &mut mark, 0, 0)
}
