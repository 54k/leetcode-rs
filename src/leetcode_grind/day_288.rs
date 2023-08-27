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

// https://leetcode.com/problems/interleaving-string/description/
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s3.len() != s1.len() + s2.len() {
        return false;
    }
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let s3 = s3.chars().collect::<Vec<_>>();

    let mut dp = vec![false; s2.len() + 1];
    for i in 0..=s1.len() {
        for j in 0..=s2.len() {
            if i == 0 && j == 0 {
                dp[j] = true;
            } else if i == 0 {
                dp[j] = dp[j - 1] && s2[j - 1] == s3[i + j - 1];
            } else if j == 0 {
                dp[j] = dp[j] && s1[i - 1] == s3[i + j - 1];
            } else {
                dp[j] = (dp[j] && s1[i - 1] == s3[i + j - 1])
                    || (dp[j - 1] && s2[j - 1] == s3[i + j - 1]);
            }
        }
    }
    dp[s2.len()]
}

// https://leetcode.com/problems/frequency-of-the-most-frequent-element/description/
pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut res = 0;
    let mut sum = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        sum += nums[j] as i64;
        while (sum + k as i64) < (nums[j] as i64 * (j as i64 - i as i64 + 1)) {
            sum -= nums[i] as i64;
            i += 1;
        }
        res = res.max(j - i + 1)
    }

    res as i32
}

// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
pub fn min_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let n = nums.len();

    let mut uniq = nums
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    uniq.sort();

    let mut res = 0;

    let mut i = 0;
    for j in 0..uniq.len() {
        while uniq[i] + n as i32 <= uniq[j] {
            i += 1;
        }
        res = res.max(j - i + 1);
    }

    n as i32 - res as i32
}

// https://leetcode.com/problems/construct-smallest-number-from-di-string/
pub fn smallest_number_i(pattern: String) -> String {
    fn is_valid(nums: &Vec<i32>, pattern: &Vec<char>) -> bool {
        for (i, &ch) in pattern.iter().enumerate() {
            if ch == 'I' {
                if nums[i] > nums[i + 1] {
                    return false;
                }
            } else if ch == 'D' {
                if nums[i] < nums[i + 1] {
                    return false;
                }
            }
        }
        true
    }

    fn generate(n: usize, used: u32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if cur.len() == n {
            res.push(cur.clone());
            return;
        }

        for i in 1..=9 {
            if ((1 << i) & used) == 0 {
                cur.push(i);
                generate(n, used | (1 << i), cur, res);
                cur.pop();
            }
        }
    }

    let pattern = pattern.chars().collect::<Vec<_>>();
    let mut res = vec![];
    generate(pattern.len() + 1, 0, &mut vec![], &mut res);

    for r in res {
        if is_valid(&r, &pattern) {
            return r
                .into_iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join("");
        }
    }
    "".to_string()
}

pub fn smallest_number_ii(pattern: String) -> String {
    let pattern = pattern.chars().collect::<Vec<_>>();
    let mut res = vec![];
    let mut stack = vec![];
    for i in 0..=pattern.len() {
        stack.push(char::from_u32('1' as u32 + i as u32).unwrap());
        if i == pattern.len() || pattern[i] == 'I' {
            let mut copy = stack.clone();
            copy.reverse();
            res.extend(copy.into_iter());
            stack.clear();
        }
    }
    res.into_iter().collect()
}

// https://leetcode.com/problems/di-string-match/description/
pub fn di_string_match(s: String) -> Vec<i32> {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = vec![0; s.len() + 1];
    let n = s.len();
    let mut lo = 0;
    let mut hi = n as i32;
    for i in 0..n {
        if s[i] == 'I' {
            ans[i] = lo;
            lo += 1;
        } else {
            ans[i] = hi;
            hi -= 1;
        }
    }
    ans[n] = lo;
    ans
}

// https://leetcode.com/problems/valid-permutations-for-di-sequence/description/
pub fn num_perms_di_sequence(s: String) -> i32 {
    let mut dp = vec![vec![0; s.len() + 1]];

    dp[s.len() - 1][s.len() - 1]
}

// https://leetcode.com/problems/build-an-array-with-stack-operations/
pub fn build_array_i(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut ans = vec![];
    let mut cur = vec![];
    let mut i = 1;
    while cur != target {
        let was_empty = cur.len() == 0;
        if i <= n {
            ans.push("Push".to_string());
            cur.push(i);
            i += 1;
        }
        if !was_empty && cur.len() > 0 {
            ans.push("Pop".to_string());
            cur.pop();
        }
    }
    ans
}

pub fn build_array_ii(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut curr = 1;
    let mut i = 0;
    while i < target.len() {
        res.push("Push".to_string());
        if target[i] != curr {
            res.push("Pop".to_string());
            i -= 1;
        }
        curr += 1;
        i += 1;
    }
    res
}
