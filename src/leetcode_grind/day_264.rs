// https://leetcode.com/problems/extra-characters-in-a-string/description/
pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    pub fn min_extra_char_top_down(s: String, dictionary: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut memo = vec![-1; s.len()];
        let dict = dictionary.into_iter().collect::<HashSet<_>>();

        fn dp(
            start: usize,
            n: usize,
            s: String,
            memo: &mut Vec<i32>,
            dict: &HashSet<String>,
        ) -> i32 {
            if start == n {
                return 0;
            }
            if memo[start] != -1 {
                return memo[start];
            }

            let mut ans = dp(start + 1, n, s.clone(), memo, dict) + 1;
            for end in start..n {
                let curr = s[start..=end].to_string();
                if dict.contains(&curr) {
                    ans = ans.min(dp(end + 1, n, s.clone(), memo, dict));
                }
            }

            memo[start] = ans;
            ans
        }

        dp(0, s.len(), s, &mut memo, &dict)
    }

    pub fn min_extra_char_bottom_up(s: String, dictionary: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let n = s.len();
        let mut dp = vec![0; s.len() + 1];
        let dict = dictionary.into_iter().collect::<HashSet<_>>();

        for start in (0..n).rev() {
            dp[start] = dp[start + 1] + 1;
            for end in start..n {
                let curr = s[start..=end].to_string();
                if dict.contains(&curr) {
                    dp[start] = dp[start].min(dp[end + 1]);
                }
            }
        }

        dp[0]
    }

    min_extra_char_bottom_up(s, dictionary)
}

// https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/description/
pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
    if k > 26 {
        return 0;
    }

    let mut ans = 0;
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut freq = vec![0; 26];

    let mut left = 0;
    for right in 0..n {
        freq[s[right] as usize - 'a' as usize] += 1;

        while freq[s[right] as usize - 'a' as usize] > 1 {
            freq[s[left] as usize - 'a' as usize] -= 1;
            left += 1;
        }

        if right - left + 1 == k as usize {
            ans += 1;
            freq[s[left] as usize - 'a' as usize] -= 1;
            left += 1;
        }
    }

    ans
}

// https://leetcode.com/problems/remove-invalid-parentheses/description/
pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    pub fn remove_invalid_parentheses_brute(s: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut valid_exprs = HashSet::new();
        let mut minimum_removed = i32::MAX;

        fn rec(
            s: &Vec<char>,
            index: usize,
            left_count: i32,
            right_count: i32,
            expression: &mut Vec<char>,
            removed_count: i32,
            minimum_removed: &mut i32,
            valid_exprs: &mut HashSet<String>,
        ) {
            if index == s.len() {
                if left_count == right_count {
                    if removed_count <= *minimum_removed {
                        let ans = expression.iter().copied().collect::<String>();
                        if removed_count < *minimum_removed {
                            valid_exprs.clear();
                            *minimum_removed = removed_count;
                        }
                        valid_exprs.insert(ans);
                    }
                }
                return;
            }

            let cur_char = s[index];

            if !['(', ')'].contains(&cur_char) {
                expression.push(cur_char);
                rec(
                    s,
                    index + 1,
                    left_count,
                    right_count,
                    expression,
                    removed_count,
                    minimum_removed,
                    valid_exprs,
                );
                expression.pop();
                return;
            }

            rec(
                s,
                index + 1,
                left_count,
                right_count,
                expression,
                removed_count + 1,
                minimum_removed,
                valid_exprs,
            );

            expression.push(cur_char);
            if cur_char == '(' {
                rec(
                    s,
                    index + 1,
                    left_count + 1,
                    right_count,
                    expression,
                    removed_count,
                    minimum_removed,
                    valid_exprs,
                )
            } else if right_count < left_count {
                rec(
                    s,
                    index + 1,
                    left_count,
                    right_count + 1,
                    expression,
                    removed_count,
                    minimum_removed,
                    valid_exprs,
                )
            }
            expression.pop();
        }

        rec(
            &s.chars().collect(),
            0,
            0,
            0,
            &mut vec![],
            0,
            &mut minimum_removed,
            &mut valid_exprs,
        );
        valid_exprs.into_iter().collect()
    }

    pub fn remove_invalid_parentheses_optimized_prunning(s: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut valid_exprs = HashSet::new();
        let (mut left, mut right) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();

        for i in 0..s.len() {
            if s[i] == '(' {
                left += 1;
            } else if s[i] == ')' {
                if left == 0 {
                    right += 1;
                }

                if left > 0 {
                    left -= 1;
                }
            }
        }

        fn rec(
            s: &Vec<char>,
            index: usize,
            left_count: i32,
            right_count: i32,
            left_rem: i32,
            right_rem: i32,
            expression: &mut Vec<char>,
            valid_exprs: &mut HashSet<String>,
        ) {
            if index == s.len() {
                if (left_rem | right_rem) == 0 {
                    valid_exprs.insert(expression.iter().copied().collect());
                }
                return;
            }

            let cur_char = s[index];

            if (cur_char == '(' && left_rem > 0) || (cur_char == ')' && right_rem > 0) {
                rec(
                    s,
                    index + 1,
                    left_count,
                    right_count,
                    left_rem - if cur_char == '(' { 1 } else { 0 },
                    right_rem - if cur_char == ')' { 1 } else { 0 },
                    expression,
                    valid_exprs,
                );
            }

            expression.push(cur_char);

            if cur_char != '(' && cur_char != ')' {
                rec(
                    s,
                    index + 1,
                    left_count,
                    right_count,
                    left_rem,
                    right_rem,
                    expression,
                    valid_exprs,
                );
            } else if cur_char == '(' {
                rec(
                    s,
                    index + 1,
                    left_count + 1,
                    right_count,
                    left_rem,
                    right_rem,
                    expression,
                    valid_exprs,
                );
            } else if left_count > right_count {
                rec(
                    s,
                    index + 1,
                    left_count,
                    right_count + 1,
                    left_rem,
                    right_rem,
                    expression,
                    valid_exprs,
                )
            }

            expression.pop();
        }

        rec(&s, 0, 0, 0, left, right, &mut vec![], &mut valid_exprs);
        valid_exprs.into_iter().collect()
    }

    remove_invalid_parentheses_optimized_prunning(s)
}

// https://leetcode.com/problems/sliding-window-maximum/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let mut q: VecDeque<(usize, i32)> = VecDeque::new();
    let mut ans = vec![];

    for right in 0..nums.len() {
        while q.len() > 0 && q[q.len() - 1].1 <= nums[right] {
            q.pop_back();
        }
        q.push_back((right, nums[right]));

        while q.len() > 0 && (q[0].0 as i32) < (right as i32) + 1 - k {
            q.pop_front();
        }

        if right as i32 + 1 >= k {
            ans.push(q[0].1);
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-window-substring/description/
pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;

    let mut ms = HashMap::new();
    let mt = t.chars().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });

    let s = s.chars().collect::<Vec<_>>();

    let (mut formed, desired) = (0, mt.len());
    let (mut window_size, mut window_substr) = (usize::MAX, "".to_string());

    let mut left = 0;
    for right in 0..s.len() {
        *ms.entry(s[right]).or_insert(0) += 1;

        if mt.contains_key(&s[right]) && ms[&s[right]] == mt[&s[right]] {
            formed += 1;
        }

        while formed == desired {
            if right - left + 1 < window_size {
                window_size = right - left + 1;
                window_substr = s[left..=right].iter().copied().collect::<String>();
            }

            *ms.entry(s[left]).or_insert(0) -= 1;
            if mt.contains_key(&s[left]) && ms[&s[left]] < mt[&s[left]] {
                formed -= 1;
            }
            left += 1;
        }
    }

    window_substr
}

mod hit_counter_hm {
    use std::collections::HashMap;

    struct HitCounter {
        max: i32,
        counter: HashMap<i32, i32>,
    }

    impl HitCounter {
        fn new() -> Self {
            Self {
                max: i32::MIN,
                counter: HashMap::new(),
            }
        }

        fn hit(&mut self, timestamp: i32) {
            self.max = timestamp;
            *self.counter.entry(timestamp).or_insert(0) += 1;
        }

        fn get_hits(&mut self, timestamp: i32) -> i32 {
            if self.max == i32::MIN {
                return 0;
            }
            let mut ans = 0;
            for i in timestamp - 299..=timestamp {
                if self.counter.contains_key(&i) {
                    ans += self.counter[&i];
                }
            }
            ans
        }
    }
}

mod hit_counter_queue {
    use std::collections::VecDeque;

    struct HitCounter {
        total: i32,
        queue: VecDeque<(i32, i32)>,
    }

    impl HitCounter {
        fn new() -> Self {
            Self {
                total: 0,
                queue: VecDeque::new(),
            }
        }

        fn hit(&mut self, timestamp: i32) {
            self.total += 1;
            if self.queue.is_empty() || self.queue[self.queue.len() - 1].0 != timestamp {
                self.queue.push_back((timestamp, 1));
                return;
            }
            let last = self.queue.pop_back().unwrap().1;
            self.queue.push_back((timestamp, last + 1));
        }

        fn get_hits(&mut self, timestamp: i32) -> i32 {
            while self.queue.len() > 0 && self.queue[0].0 <= timestamp - 300 {
                self.total -= self.queue.pop_front().unwrap().1;
            }
            self.total
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        println!("{:?}", remove_invalid_parentheses("()())()".to_string()));
    }

    #[test]
    fn test_max_sliding_window() {
        println!(
            "{:?}",
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        ); // [3,3,5,5,6,7]
    }

    #[test]
    fn test_min_window() {
        println!(
            "{}",
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        ); // "BANC"
    }
}
