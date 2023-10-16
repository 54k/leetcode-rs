// https://leetcode.com/problems/pascals-triangle-ii
pub fn get_row_bruteforce(row_index: i32) -> Vec<i32> {
    fn get_num(row: i32, col: i32) -> i32 {
        if row == 0 || col == 0 || row == col {
            return 1;
        }
        get_num(row - 1, col - 1) + get_num(row - 1, col)
    }

    let mut ans = vec![];
    for i in 0..=row_index {
        ans.push(get_num(row_index, i));
    }
    ans
}

// https://leetcode.com/problems/pascals-triangle-ii/description
pub fn get_row_mem_optimized_dp(row_index: i32) -> Vec<i32> {
    let mut row = vec![1];
    for i in 0..row_index as usize {
        for j in (1..=i).rev() {
            row[j] += row[j - 1];
        }
        row.push(1);
    }
    row
}

// https://leetcode.com/problems/painting-the-walls/
pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = cost.len();
    const INF: i32 = 1000_000_000;

    let mut dp = vec![INF; n + 1];
    dp[0] = 0;

    for i in (0..n).rev() {
        let mut next = vec![0; n + 1];
        for remain in 1..=n {
            let paint = cost[i] + dp[0.max(remain as i32 - 1 - time[i]) as usize];
            let not_paint = dp[remain];
            next[remain] = paint.min(not_paint);
        }
        dp = next;
    }
    dp[n]
}

// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/description/
pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
    const MOD: i32 = 1000_000_007;
    let arr_len = arr_len.min(steps) as usize;
    let steps = steps as usize;
    let mut prev = vec![0; arr_len];
    prev[0] = 1;

    for _ in 1..=steps {
        let mut next = vec![0; arr_len];
        for pos in 0..arr_len {
            let mut ans = prev[pos];

            let mut left = 0;
            if pos > 0 {
                left = prev[pos - 1];
            }

            let mut right = 0;
            if pos < arr_len - 1 {
                right = prev[pos + 1];
            }

            ans = ((ans + left) % MOD + right) % MOD;
            next[pos] = ans;
        }
        prev = next;
    }

    prev[0]
}
