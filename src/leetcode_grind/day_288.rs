// https://leetcode.com/problems/frog-jump/description/
pub fn can_cross_i(stones: Vec<i32>) -> bool {
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

pub fn can_cross_ii(stones: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut mark = HashMap::new();
    for (i, &s) in stones.iter().enumerate() {
        mark.insert(s, i);
    }

    let mut dp = vec![vec![false; 2001]; 2001];
    dp[0][0] = true;

    for index in 0..stones.len() {
        for prev_jump in 0..=stones.len() as i32 {
            if dp[index][prev_jump as usize] {
                if mark.contains_key(&(stones[index] + prev_jump)) {
                    dp[mark[&(stones[index] + prev_jump)]][prev_jump as usize] = true;
                }

                if mark.contains_key(&(stones[index] + prev_jump + 1)) {
                    dp[mark[&(stones[index] + prev_jump + 1)]][prev_jump as usize + 1] = true;
                }

                if mark.contains_key(&(stones[index] + prev_jump - 1)) {
                    dp[mark[&(stones[index] + prev_jump - 1)]][prev_jump as usize - 1] = true;
                }
            }
        }
    }

    for index in 0..=stones.len() {
        if dp[stones.len() - 1][index] {
            return true;
        }
    }
    false
}

// https://leetcode.com/problems/design-a-stack-with-increment-operation/description/
struct CustomStack {
    stack: Vec<(i32, i32)>,
    max_size: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            stack: Vec::new(),
            max_size: maxSize as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push((x, 0));
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            return -1;
        }
        let top = self.stack.pop().unwrap();
        self.increment(self.stack.len() as i32, top.1);
        top.0 + top.1
    }

    fn increment(&mut self, k: i32, val: i32) {
        if self.stack.is_empty() {
            return;
        }
        let n = self.stack.len();
        if n < k as usize {
            self.stack[n - 1].1 += val;
        } else {
            self.stack[k as usize - 1].1 += val;
        }
    }
}
