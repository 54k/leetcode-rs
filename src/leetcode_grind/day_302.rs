// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/description
pub fn count_orders_top_down(n: i32) -> i32 {
    const MOD: i64 = 1000000007;

    fn dp(unpicked: i32, undelivered: i32, memo: &mut Vec<Vec<i64>>) -> i64 {
        if unpicked == 0 && undelivered == 0 {
            return 1;
        }

        if unpicked < 0 || undelivered < 0 || undelivered < unpicked {
            return 0;
        }

        if memo[unpicked as usize][undelivered as usize] > -1 {
            return memo[unpicked as usize][undelivered as usize];
        }

        let mut ans: i64 = 0;
        ans += unpicked as i64 * dp(unpicked - 1, undelivered, memo);
        ans %= MOD;

        ans += (undelivered - unpicked) as i64 * dp(unpicked, undelivered - 1, memo);
        ans %= MOD;

        memo[unpicked as usize][undelivered as usize] = ans;
        ans
    }

    let mut memo = vec![vec![-1; n as usize + 1]; n as usize + 1];
    dp(n, n, &mut memo) as i32
}

pub fn count_orders_bottom_up(n: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let mut dp = vec![vec![0; n as usize + 1]; n as usize + 1];
    for unpicked in 0..=n as usize {
        for undelivered in unpicked..=n as usize {
            if unpicked == 0 && undelivered == 0 {
                dp[unpicked][undelivered] = 1;
                continue;
            }

            if unpicked > 0 {
                dp[unpicked][undelivered] += unpicked as i64 * dp[unpicked - 1][undelivered] as i64;
            }
            dp[unpicked][undelivered] %= MOD;

            if undelivered > unpicked {
                dp[unpicked][undelivered] +=
                    (undelivered - unpicked) as i64 * dp[unpicked][undelivered - 1] as i64;
            }
            dp[unpicked][undelivered] %= MOD;
        }
    }
    dp[n as usize][n as usize] as i32
}

// https://leetcode.com/problems/ternary-expression-parser/description/
pub fn parse_ternary_tree(expression: String) -> String {
    use std::{cell::RefCell, rc::Rc};
    struct TreeNode {
        val: char,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    }

    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn construct_tree(expression: &Vec<char>, index: &mut usize) -> Node {
        let mut root = TreeNode {
            val: expression[*index],
            left: None,
            right: None,
        };

        if *index == expression.len() - 1 {
            return Some(Rc::new(RefCell::new(root)));
        }

        *index += 1;
        if expression[*index] == '?' {
            *index += 1;
            root.left = construct_tree(expression, index);
            *index += 1;
            root.right = construct_tree(expression, index);
        }

        Some(Rc::new(RefCell::new(root)))
    }

    let mut index = 0;
    let mut root = construct_tree(&expression.chars().collect(), &mut index);

    while let Some(r) = root.clone() {
        let r = r.borrow();
        if r.left.is_some() && r.right.is_some() {
            if r.val == 'T' {
                root = r.left.clone();
            } else {
                root = r.right.clone();
            }
        } else {
            break;
        }
    }

    root.unwrap().borrow().val.to_string()
}

pub fn parse_ternary_recursion(expression: String) -> String {
    let expression = expression.chars().collect::<Vec<_>>();
    fn solve(expression: &Vec<char>, left: usize, right: usize) -> Vec<char> {
        if left == right {
            return expression[left..left + 1].iter().copied().collect();
        }

        let mut question_mark_index = left;
        while expression[question_mark_index] != '?' {
            question_mark_index += 1;
        }

        let mut ahead_colon_index = question_mark_index + 1;
        let mut count = 1;
        while count != 0 {
            if expression[ahead_colon_index] == '?' {
                count += 1;
            } else if expression[ahead_colon_index] == ':' {
                count -= 1;
            }
            ahead_colon_index += 1;
        }

        if expression[left] == 'T' {
            solve(expression, question_mark_index + 1, ahead_colon_index - 2)
        } else {
            solve(expression, ahead_colon_index, right)
        }
    }

    solve(&expression, 0, expression.len() - 1)
        .into_iter()
        .collect()
}

pub fn parse_ternary_iterative(expression: String) -> String {
    let mut i = 0;
    let expression = expression.chars().collect::<Vec<_>>();
    loop {
        if expression[i] != 'T' && expression[i] != 'F' {
            return expression[i].to_string();
        }

        if i == expression.len() - 1 {
            return expression[i].to_string();
        }

        if expression[i + 1] == ':' {
            return expression[i].to_string();
        }

        if expression[i] == 'T' {
            i += 2;
        } else {
            let mut count = 1;
            i += 2;
            while count != 0 {
                if expression[i] == ':' {
                    count -= 1;
                } else if expression[i] == '?' {
                    count += 1;
                }
                i += 1;
            }
        }
    }
}

#[test]
fn test_ternary_parser() {
    let res = parse_ternary_tree("T?2:3".to_string());
    println!("{res}");
}
