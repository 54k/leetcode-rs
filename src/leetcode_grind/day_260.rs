// https://leetcode.com/problems/strange-printer/description/
pub fn strange_printer(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![n as i32; n]; n];
    for length in 1..=n {
        for left in 0..=n - length {
            let right = left + length - 1;
            let mut j = -1;

            for i in left..right {
                if s[i] != s[right] && j == -1 {
                    j = i as i32;
                }
                if j != -1 {
                    dp[left][right] =
                        dp[left][right].min(1 + dp[j as usize][i] + dp[i + 1][right]) as i32;
                }
            }

            if j == -1 {
                dp[left][right] = 0;
            }
        }
    }

    dp[0][n - 1] + 1
}

// https://leetcode.com/problems/find-the-celebrity/description/
// pub fn find_celebrity(&self, n: i32) -> i32 {
//     fn is_celeb(this: &Solution, n: i32, i: i32) -> bool {
//         for j in 0..n {
//             if i == j {
//                 continue;
//             }
//             if this.knows(i, j) || !this.knows(j, i) {
//                 return false;
//             }
//         }
//         true
//     }
//     let mut celeb_candidate = 0;
//     for i in 0..n {
//         if self.knows(celeb_candidate, i) {
//             celeb_candidate = i;
//         }
//     }

//     if is_celeb(self, n, celeb_candidate) {
//         return celeb_candidate;
//     }
//     -1
// }

// https://leetcode.com/problems/number-of-employees-who-met-the-target/
pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.into_iter().filter(|x| *x >= target).count() as i32
}

// https://leetcode.com/problems/count-complete-subarrays-in-an-array/description/
pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let total = nums.iter().copied().collect::<HashSet<_>>();
    let mut ans = 0;

    for i in 0..nums.len() {
        let mut curr = HashSet::new();
        for j in i..nums.len() {
            curr.insert(nums[j]);
            if curr.len() == total.len() {
                ans += 1;
            }
        }
    }
    ans
}

// https://leetcode.com/problems/shortest-string-that-contains-three-strings/description/
pub fn minimum_string(a: String, b: String, c: String) -> String {
    use std::cmp::Ordering;

    fn get_min_string(a: String, b: String) -> String {
        if a.len() < b.len() || (a.len() == b.len() && a.cmp(&b) == Ordering::Less) {
            a
        } else {
            b
        }
    }

    fn add_two_strings(a: String, b: String) -> String {
        if b.find(&a).is_some() {
            return b;
        }

        let n = a.len();
        for i in 0..n {
            let t1 = a[i..].to_owned();
            let t2 = b[0..t1.len().min(b.len())].to_owned();
            if t1 == t2 {
                return a + &b[t1.len()..];
            }
        }

        a + &b[0..]
    }

    fn solve(a: String, b: String, c: String) -> String {
        let t1 = add_two_strings(a.clone(), b.clone());
        let t2 = add_two_strings(b.clone(), a.clone());

        let res1 = get_min_string(
            add_two_strings(t1.clone(), c.clone()),
            add_two_strings(c.clone(), t1.clone()),
        );

        let res2 = get_min_string(
            add_two_strings(t2.clone(), c.clone()),
            add_two_strings(c.clone(), t2.clone()),
        );

        get_min_string(res1.clone(), res2.clone())
    }

    let res = get_min_string(
        solve(a.clone(), b.clone(), c.clone()),
        solve(b.clone(), c.clone(), a.clone()),
    );

    get_min_string(res, solve(c.clone(), a.clone(), b.clone()))
}

// https://leetcode.com/problems/stepping-numbers/description/
pub fn count_stepping_numbers(low: i32, high: i32) -> Vec<i32> {
    pub fn count_stepping_numbers_dfs(low: i32, high: i32) -> Vec<i32> {
        fn dfs(cur: i64, res: &mut Vec<i32>, low: i32, high: i32) {
            if cur >= low as i64 && cur <= high as i64 {
                res.push(cur as i32);
            }

            if cur > high as i64 || cur == 0 {
                return;
            }

            let last = cur % 10;
            if last != 0 {
                dfs(cur * 10 + last - 1, res, low, high);
            }
            if last != 9 {
                dfs(cur * 10 + last + 1, res, low, high)
            }
        }

        let mut res = vec![];
        for i in 0..=9 {
            dfs(i, &mut res, low, high);
        }

        res.sort();
        res
    }

    count_stepping_numbers_dfs(low, high)
}
