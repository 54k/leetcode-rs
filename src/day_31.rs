// https://claytonjwong.github.io/The-ART-of-Dynamic-Programming/
// https://leetcode.com/problems/minimum-falling-path-sum/solutions/186646/the-art-of-dynamic-programming/
#[allow(dead_code)]
pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    const INF: i32 = 1000000007;

    // All possibilities are considered via top-down brute-force depth-first-search
    fn all(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        fn go(A: &Vec<Vec<i32>>, i: i32, j: i32, m: i32, n: i32) -> i32 {
            if j < 0 || j == n {
                return INF;
            }
            if i == 0 {
                return A[i as usize][j as usize];
            }

            let a = go(A, i - 1, j - 1, n, m);
            let b = go(A, i - 1, j, n, m);
            let c = go(A, i - 1, j + 1, n, m);

            A[i as usize][j as usize] + a.min(b).min(c)
        }

        let mut best = INF;
        for j in 0..n {
            best = best.min(go(&matrix, m - 1, j, m, n));
        }
        best
    }

    // Remember each sub-problem's optimal solution via a DP memo
    fn remember(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut memo = HashMap::new();
        fn go(
            A: &Vec<Vec<i32>>,
            i: i32,
            j: i32,
            m: i32,
            n: i32,
            memo: &mut HashMap<String, i32>,
        ) -> i32 {
            if j < 0 || j == n {
                return INF;
            }
            if i == 0 {
                return A[i as usize][j as usize];
            }

            let k = format!("{}:{}", i, j);
            if !memo.contains_key(&k) {
                let a = go(A, i - 1, j - 1, n, m, memo);
                let b = go(A, i - 1, j, n, m, memo);
                let c = go(A, i - 1, j + 1, n, m, memo);

                memo.insert(k.clone(), A[i as usize][j as usize] + a.min(b).min(c));
            }

            *memo.get(&k).unwrap()
        }

        let mut best = INF;
        for j in 0..n {
            best = best.min(go(&matrix, m - 1, j, m, n, &mut memo));
        }
        best
    }

    // Turn the top-down solution upside-down to create the bottom-up solution
    fn turn(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        for i in 1..m {
            for j in 0..n {
                let a = if 0 < j {
                    matrix[i as usize - 1][j as usize - 1]
                } else {
                    INF
                };
                let b = matrix[i as usize - 1][j as usize];
                let c = if j < n - 1 {
                    matrix[i as usize - 1][j as usize + 1]
                } else {
                    INF
                };
                matrix[i as usize][j as usize] += a.min(b).min(c);
            }
        }

        let last_row = &matrix[m as usize - 1];
        *last_row.iter().min().unwrap()
    }

    turn(matrix)
}

#[allow(dead_code)]
pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut start = 0;
    let mut zeros_count = 0;

    for end in 0..nums.len() {
        if nums[end] == 0 {
            zeros_count += 1;
        }
        if zeros_count > k {
            res = res.max(end - start);
            while nums[start] == 1 {
                start += 1;
            }
            start += 1;
            zeros_count -= 1;
        }
    }
    res.max(nums.len() - start) as i32
}

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashSet;

    let s = s.chars().collect::<Vec<char>>();
    let mut seen = HashSet::new();
    let mut ans = 0;
    let mut start = 0;
    for end in 0..s.len() {
        while seen.contains(&s[end]) {
            seen.remove(&s[start]);
            start += 1;
        }
        seen.insert(s[end]);
        ans = ans.max(end - start + 1);
    }

    ans.max(s.len() - start) as i32
}

#[allow(dead_code)]
pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    const INF: usize = 1000000007;

    let s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().fold(HashMap::<char, i32>::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let desired = t.len();

    let mut formed = 0;
    let mut cur_cnt = HashMap::new();

    let mut start = 0;
    let mut end = 0;

    let mut ans = (INF, 0, 0);

    while end < s.len() {
        let ch = s[end];

        *cur_cnt.entry(ch).or_insert(0) += 1;
        if t.contains_key(&ch) && t.get(&ch) == cur_cnt.get(&ch) {
            formed += 1;
        }

        while start <= end && desired == formed {
            let ch = s[start];

            if ans.0 > end - start + 1 {
                ans = (end - start + 1, start, end);
            }

            cur_cnt.entry(ch).and_modify(|c| *c -= 1);

            if t.contains_key(&ch) && t.get(&ch) > cur_cnt.get(&ch) {
                formed -= 1;
            }
            start += 1;
        }

        end += 1;
    }

    if ans.0 == INF {
        "".to_string()
    } else {
        s[ans.1..=ans.2].into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test111() {
        println!("{}", longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)); // 6
    }

    #[test]
    fn test112() {
        println!("{}", length_of_longest_substring("abcabcbb".to_string())); // 3
        println!("{}", length_of_longest_substring("bbbbb".to_string())); // 1
        println!("{}", length_of_longest_substring("pwwkew".to_string())); // 3
        println!("{}", length_of_longest_substring("dvdf".to_string())); // 3
    }

    #[test]
    fn test113() {
        println!(
            "{}",
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        ); // BANC
        println!("{}", min_window("a".to_string(), "a".to_string())); // a
    }
}
