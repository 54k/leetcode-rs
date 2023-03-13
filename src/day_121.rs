// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/
use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/symmetric-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_symmetric(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            true
        } else if left.is_some() && right.is_some() {
            let left = left.unwrap();
            let left = left.borrow();
            let right = right.unwrap();
            let right = right.borrow();

            left.val == right.val
                && is_symmetric(left.left.clone(), right.right.clone())
                && is_symmetric(left.right.clone(), right.left.clone())
        } else {
            false
        }
    }
    is_symmetric(root.clone(), root)
}

// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/
// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/editorial/
pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
    fn bottom_up(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; piles.len() + 1];
        for i in 1..=piles.len() as usize {
            for coins in 0..=k as usize {
                let mut current_sum = 0;
                for current_coins in 0..=piles[i - 1].len().min(coins) {
                    if current_coins > 0 {
                        current_sum += piles[i - 1][current_coins - 1];
                    }
                    dp[i][coins] = dp[i][coins].max(dp[i - 1][coins - current_coins] + current_sum);
                }
            }
        }
        dp[piles.len()][k as usize]
    }

    fn top_down(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        fn rec(piles: &Vec<Vec<i32>>, i: usize, coins: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if i == 0 {
                return 0;
            }
            if dp[i][coins] != -1 {
                return dp[i][coins];
            }
            let mut current_sum = 0;
            for current_coins in 0..=piles[i - 1].len().min(coins) {
                if current_coins > 0 {
                    current_sum += piles[i - 1][current_coins - 1];
                }
                dp[i][coins] =
                    dp[i][coins].max(rec(piles, i - 1, coins - current_coins, dp) + current_sum);
            }
            dp[i][coins]
        }
        let mut dp = vec![vec![-1; k as usize + 1]; piles.len() + 1];
        rec(&piles, piles.len(), k as usize, &mut dp)
    }

    top_down(piles, k)
}

// https://leetcode.com/problems/coin-change-ii/description/
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut dp = vec![0; amount as usize + 1];
    dp[0] = 1;
    for coin in 0..coins.len() {
        for amt in 0..=amount as usize {
            if (coins[coin] as usize) <= amt {
                dp[amt] += dp[amt - coins[coin] as usize];
            }
        }
    }
    dp[amount as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test344() {
        println!(
            "{}",
            max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2)
        ); // 101
    }

    #[test]
    fn test345() {
        println!("{}", change(5, vec![1, 2, 5])); // 4
        println!("{}", change(3, vec![2])); // 0
        println!("{}", change(10, vec![10])); // 1
    }
}
