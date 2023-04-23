// https://leetcode.com/problems/restore-the-array/
pub fn number_of_arrays(s: String, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        if s[i] == '0' {
            continue;
        }
        for j in i..s.len() {
            let num = s[i..=j].iter().collect::<String>().parse::<i64>().unwrap();
            if num > k as i64 {
                break;
            }
            dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
        }
    }
    dp[s.len()] as i32
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    fn top_down(k: i32, prices: Vec<i32>) -> i32 {
        fn rec(
            dp: &mut Vec<Vec<Vec<i32>>>,
            prices: &Vec<i32>,
            i: usize,
            hold: usize,
            remain: usize,
        ) -> i32 {
            if i == prices.len() || remain == 0 {
                return 0;
            }
            if dp[i][hold][remain] > -1 {
                return dp[i][hold][remain];
            }
            let mut ans = rec(dp, prices, i + 1, hold, remain);
            if hold == 1 {
                ans = ans.max(prices[i] + rec(dp, prices, i + 1, 0, remain - 1));
            } else {
                ans = ans.max(-prices[i] + rec(dp, prices, i + 1, 1, remain));
            }
            dp[i][hold][remain] = ans;
            dp[i][hold][remain]
        }
        let mut dp = vec![vec![vec![-1; k as usize + 1]; 2]; prices.len()];
        rec(&mut dp, &prices, 0, 0, k as usize)
    }
    fn bottom_up(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![vec![0; k as usize + 1]; 2]; prices.len() + 1];
        for i in (0..prices.len()).rev() {
            for remain in 1..=k as usize {
                for holding in 0..2 {
                    let mut ans = dp[i + 1][holding][remain];
                    if holding == 1 {
                        ans = ans.max(prices[i] + dp[i + 1][0][remain - 1]);
                    } else {
                        ans = ans.max(-prices[i] + dp[i + 1][1][remain]);
                    }
                    dp[i][holding][remain] = ans;
                }
            }
        }
        dp[0][0][k as usize]
    }
    bottom_up(k, prices)
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
pub fn max_profit2(prices: Vec<i32>, fee: i32) -> i32 {
    fn my_approach(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = vec![vec![0; 2]; prices.len() + 1];
        for i in (0..prices.len()).rev() {
            for holding in 0..2 {
                let mut ans = dp[i + 1][holding]; // skip
                if holding == 1 {
                    ans = ans.max(prices[i] - fee + dp[i + 1][0]); // sell
                } else {
                    ans = ans.max(-prices[i] + dp[i + 1][1]); // buy
                }
                dp[i][holding] = ans;
            }
        }
        dp[0][0]
    }
    fn leetcode_approach(prices: Vec<i32>, fee: i32) -> i32 {
        let mut free = vec![0; prices.len()];
        let mut hold = vec![0; prices.len()];
        hold[0] = -prices[0];
        for i in 0..prices.len() {
            hold[i] = hold[i - 1].max(free[i - 1] - prices[i]);
            free[i] = free[i - 1].max(hold[i - 1] + prices[i] - fee);
        }
        free[prices.len() - 1]
    }
    leetcode_approach(prices, fee)
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
pub fn max_profit3(prices: Vec<i32>) -> i32 {
    fn state_machine_approach(prices: Vec<i32>) -> i32 {
        let mut sold = i32::MIN;
        let mut held = i32::MIN;
        let mut reset = 0;

        for price in prices {
            let pre_sold = sold;
            sold = held + price;
            held = held.max(reset - price);
            reset = reset.max(pre_sold);
        }
        sold.max(reset)
    }
    fn dp_approach(prices: Vec<i32>) -> i32 {
        let mut dp = vec![0; prices.len() + 2];
        for i in (0..prices.len()).rev() {
            let mut c1 = 0;
            for j in i + 1..prices.len() {
                let profit = prices[j] - prices[i] + dp[j + 2];
                c1 = c1.max(profit);
            }
            dp[i] = c1.max(dp[i + 1]);
        }
        dp[0]
    }
    dp_approach(prices)
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/description/
pub fn max_profit4(prices: Vec<i32>) -> i32 {
    let mut dp = vec![vec![vec![0; 3]; 2]; prices.len() + 1];
    for i in (0..prices.len()).rev() {
        for hold in 0..2 {
            for remain in 1..=2 {
                let mut profit = dp[i + 1][hold][remain];
                if hold == 1 {
                    profit = profit.max(prices[i] + dp[i + 1][0][remain - 1]);
                } else {
                    profit = profit.max(-prices[i] + dp[i + 1][1][remain]);
                }
                dp[i][hold][remain] = profit;
            }
        }
    }
    dp[0][0][2]
}

// https://leetcode.com/problems/unique-paths-ii/editorial/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let n = obstacle_grid.len();
    let m = obstacle_grid[0].len();
    let mut dp = vec![vec![0; obstacle_grid[0].len()]; n];
    if obstacle_grid[0][0] == 0 {
        dp[0][0] = 1;
    }
    for i in 1..n {
        if obstacle_grid[i][0] == 0 {
            dp[i][0] = dp[i - 1][0];
        }
    }
    for i in 1..m {
        if obstacle_grid[0][i] == 0 {
            dp[0][i] = dp[0][i - 1];
        }
    }
    for i in 1..n {
        for j in 1..m {
            if obstacle_grid[i][j] == 0 {
                dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }

    dp[n - 1][m - 1]
}

// https://leetcode.com/problems/minimum-falling-path-sum/description/
pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; matrix.len() + 1];
    for row in (0..matrix.len()).rev() {
        let mut current_row = vec![0; matrix.len() + 1];
        for col in 0..matrix.len() {
            if col == 0 {
                current_row[col] = dp[col].min(dp[col + 1]) + matrix[row][col];
            } else if col == matrix.len() - 1 {
                current_row[col] = dp[col].min(dp[col - 1]) + matrix[row][col];
            } else {
                current_row[col] = dp[col].min(dp[col + 1].min(dp[col - 1])) + matrix[row][col];
            }
        }
        dp = current_row;
    }
    let mut ans = i32::MAX;
    for i in 0..matrix.len() {
        ans = ans.min(dp[i]);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test456() {
        println!(
            "{}",
            number_of_arrays("1111111111111".to_string(), 1000000000)
        ); // 1
        println!("{}", number_of_arrays("1000".to_string(), 10000)); // 1
        println!("{}", number_of_arrays("1317".to_string(), 2000)); // 8
    }

    #[test]
    fn test457() {
        println!("{}", max_profit(2, vec![3, 2, 6, 5, 0, 3])); // 7
    }

    #[test]
    fn test458() {
        println!("{}", max_profit2(vec![1, 3, 2, 8, 4, 9], 2)); // 8
        println!("{}", max_profit2(vec![1, 3, 7, 5, 10, 3], 3)); // 6
    }

    #[test]
    fn test459() {
        println!("{}", max_profit3(vec![1, 2, 3, 0, 2])); // 3
        println!("{}", max_profit3(vec![1])); // 0
    }

    #[test]
    fn test460() {
        println!("{}", max_profit4(vec![3, 3, 5, 0, 0, 3, 1, 4])); // 6
        println!("{}", max_profit4(vec![1, 2, 3, 4, 5])); // 4
        println!("{}", max_profit4(vec![7, 6, 4, 3, 1])); // 0
    }

    #[test]
    fn test461() {
        println!(
            "{}",
            min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]])
        ); // 13
    }
}
