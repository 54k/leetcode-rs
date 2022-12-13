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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test111() {
        println!("{}", longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)); // 6
    }
}
