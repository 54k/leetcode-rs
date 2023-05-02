// https://leetcode.com/problems/sign-of-the-product-of-an-array/
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut min = 0;
    for num in nums {
        if num < 0 {
            min += 1;
        } else if num == 0 {
            return 0;
        }
    }
    if min % 2 == 1 {
        -1
    } else {
        1
    }
}

// https://leetcode.com/problems/walls-and-gates/description/
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    use std::collections::VecDeque;
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q = VecDeque::new();
    for i in 0..rooms.len() {
        for j in 0..rooms[i].len() {
            if rooms[i][j] == 0 {
                q.push_back((i as i32, j as i32));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        for dir in DIRS {
            let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);
            if x < 0
                || x >= rooms.len() as i32
                || y < 0
                || y >= rooms[0].len() as i32
                || rooms[x as usize][y as usize] != i32::MAX
            {
                continue;
            }
            rooms[x as usize][y as usize] = rooms[i as usize][j as usize] + 1;
            q.push_back((x, y));
        }
    }
}

// https://leetcode.com/problems/design-an-atm-machine/
struct ATM {
    store: [(i32, i64); 5],
}

impl ATM {
    fn new() -> Self {
        Self {
            store: [(500, 0), (200, 0), (100, 0), (50, 0), (20, 0)],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, cnt) in banknotes_count.into_iter().rev().enumerate() {
            self.store[i].1 += cnt as i64;
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut ans = vec![0; 5];
        let mut store = self.store.clone();

        for (i, (amt, amt_cnt)) in store.iter_mut().enumerate() {
            if *amt > amount || *amt_cnt == 0 {
                continue;
            }

            let cnt = (amount as i64 / *amt as i64).min(*amt_cnt);
            *amt_cnt -= cnt;

            ans[5 - i - 1] = cnt as i32;
            amount -= cnt as i32 * *amt;
        }

        if amount > 0 {
            return vec![-1];
        }
        self.store = store;
        ans
    }
}

// https://leetcode.com/problems/target-sum/description/
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    fn using_recursion(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(
            nums: &Vec<i32>,
            target: i32,
            sum: i32,
            i: usize,
            total: i32,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == nums.len() {
                if sum == target {
                    1
                } else {
                    0
                }
            } else {
                if cache[i][(sum + total) as usize] == i32::MIN {
                    let ads = dfs(nums, target, sum + nums[i], i + 1, total, cache);
                    let rems = dfs(nums, target, sum - nums[i], i + 1, total, cache);
                    cache[i][(sum + total) as usize] = ads + rems;
                }
                cache[i][(sum + total) as usize]
            }
        }
        let total = nums.iter().copied().sum::<i32>();
        let mut cache = vec![vec![i32::MIN; total as usize * 2 + 1]; nums.len()];
        dfs(&nums, target, 0, 0, total, &mut cache)
    }

    fn using_dp(nums: Vec<i32>, target: i32) -> i32 {
        let total = nums.iter().copied().sum::<i32>();
        let mut dp = vec![vec![0; total as usize * 2 + 1]; nums.len()];
        dp[0][(nums[0] + total) as usize] = 1;
        dp[0][(-nums[0] + total) as usize] += 1;

        for i in 1..nums.len() {
            for sum in -total..=total {
                if dp[i - 1][(sum + total) as usize] > 0 {
                    dp[i][(sum + nums[i] + total) as usize] += dp[i - 1][(sum + total) as usize];
                    dp[i][(sum - nums[i] + total) as usize] += dp[i - 1][(sum + total) as usize];
                }
            }
        }

        if target.abs() > total {
            0
        } else {
            dp[nums.len() - 1][(target + total) as usize]
        }
    }
    using_dp(nums, target)
}

// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/description/
pub fn longest_substring(s: String, k: i32) -> i32 {
    fn brute_force(s: String, k: i32) -> i32 {
        fn is_okay(v: &Vec<i32>, k: i32) -> bool {
            for i in 0..26 {
                if v[i] == 0 {
                    continue;
                }
                if v[i] < k {
                    return false;
                }
            }
            true
        }
        let mut ans = 0;
        let s = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            let mut freq = vec![0; 26];
            for j in i..s.len() {
                freq[s[j] as usize - 'a' as usize] += 1;

                if is_okay(&freq, k) {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as i32
    }
    fn divide_and_conquer(s: String, k: i32) -> i32 {
        fn go(s: &Vec<char>, start: usize, end: usize, k: i32) -> i32 {
            let mut freq = vec![0; 26];
            for i in start..end {
                freq[s[i] as usize - 'a' as usize] += 1;
            }
            for mid in start..end {
                if freq[s[mid] as usize - 'a' as usize] >= k {
                    continue;
                }
                let mut mid_next = mid + 1;
                while mid_next < s.len() && freq[s[mid_next] as usize - 'a' as usize] < k {
                    mid_next += 1;
                }
                return go(s, start, mid, k).max(go(s, mid_next, end, k));
            }
            end as i32 - start as i32
        }
        go(&s.chars().collect(), 0, s.len(), k)
    }
    fn sliding_window(s: String, k: i32) -> i32 {
        let uniq = s.chars().collect::<std::collections::HashSet<char>>().len();
        let s = s.chars().collect::<Vec<_>>();
        let mut result = 0;
        for curr_unique in 1..=uniq {
            let mut freq = vec![0; 26];
            let mut window_start = 0;
            let mut window_end = 0;
            let mut unique = 0;
            let mut idx = 0;
            let mut count_at_least_k = 0;

            while window_end < s.len() {
                if unique <= curr_unique {
                    idx = s[window_end] as usize - 'a' as usize;
                    if freq[idx] == 0 {
                        unique += 1;
                    }
                    freq[idx] += 1;
                    if freq[idx] == k {
                        count_at_least_k += 1;
                    }
                    window_end += 1;
                } else {
                    idx = s[window_start] as usize - 'a' as usize;
                    if freq[idx] == k {
                        count_at_least_k -= 1;
                    }
                    freq[idx] -= 1;
                    if freq[idx] == 0 {
                        unique -= 1;
                    }
                    window_start += 1;
                }
                if unique == curr_unique && unique == count_at_least_k {
                    result = result.max(window_end - window_start);
                }
            }
        }
        result as i32
    }
    sliding_window(s, k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test474() {
        println!("{}", array_sign(vec![-1, 1, -1, 1, -1]));
    }

    #[test]
    fn test475() {
        let mut rooms = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        walls_and_gates(&mut rooms);
        println!("{:?}", rooms);
    }

    #[test]
    fn test476() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        println!("{:?}", atm.withdraw(600));
        atm.deposit(vec![0, 1, 0, 1, 1]);
        println!("{:?}", atm.withdraw(600));
        println!("{:?}", atm.withdraw(550));

        let mut atm = ATM::new();
        atm.deposit(vec![0, 10, 0, 3, 0]);
        println!("{:?}", atm.withdraw(500)); // [0,2,0,2,0]
    }

    #[test]
    fn test478() {
        println!("{}", find_target_sum_ways(vec![1, 1, 1, 1, 1], 3)); // 5
    }

    #[test]
    fn test479() {
        println!("{}", longest_substring("ababbc".to_string(), 2)); // 5
    }
}
