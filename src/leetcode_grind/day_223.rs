// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let n = prices.len();
    let (mut free, mut hold) = (0, -prices[0]);

    for i in 1..n {
        let tmp = hold;
        hold = hold.max(free - prices[i]);
        free = free.max(tmp + prices[i] - fee);
    }

    free
}

// https://leetcode.com/problems/single-number-iii/description/
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut mask = 0;
    for &num in &nums {
        mask ^= num;
    }

    let diff = mask & (-mask);
    let mut x = 0;

    for &num in &nums {
        if (num & diff) != 0 {
            x ^= num;
        }
    }

    vec![x, mask ^ x]
}

// https://leetcode.com/problems/maximum-students-taking-exam/description/
// https://leetcode.com/problems/maximum-students-taking-exam/solutions/503686/a-simple-tutorial-on-this-bitmasking-problem/
pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    fn pop_count(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            ans += 1;
            n &= n - 1;
        }
        ans
    }

    let (m, n) = (seats.len(), seats[0].len());

    let mut valid_rows = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            valid_rows[i] = (valid_rows[i] << 1) + if seats[i][j] == '.' { 1 } else { 0 };
        }
    }

    let state_size = 1 << n; // There are 2^n states for n columns in binary format
    let mut dp = vec![vec![-1i32; state_size + 1]; m];

    let mut ans = 0;

    for i in 0..m {
        for j in 0..state_size {
            // (j & valid) == j: check if j is a subset of valid
            // !(j & (j >> 1)): check if there is no adjancent students in the row
            if (j & valid_rows[i]) == j && (j & (j >> 1)) == 0 {
                if i == 0 {
                    dp[i][j] = pop_count(j as i32);
                } else {
                    for k in 0..state_size {
                        // !(j & (k >> 1)): no students in the upper left positions
                        // !((j >> 1) & k): no students in the upper right positions
                        // dp[i-1][k] != -1: the previous state is valid
                        if (j & (k >> 1)) == 0 && ((j >> 1) & k) == 0 && dp[i - 1][k] != -1 {
                            // println!("k {:#018b} j {:#018b}", k, j);
                            dp[i][j] = dp[i][j].max(dp[i - 1][k] + pop_count(j as i32));
                        }
                    }
                }

                ans = ans.max(dp[i][j]);
            }
        }
    }
    ans
}
