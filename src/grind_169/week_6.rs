use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, height: usize, levels: &mut Vec<Vec<i32>>) {
            if let Some(r) = root {
                if levels.len() == height {
                    levels.push(vec![]);
                }
                dfs(r.as_ref().borrow().left.clone(), height + 1, levels);
                dfs(r.as_ref().borrow().right.clone(), height + 1, levels);
                levels[height].push(r.as_ref().borrow().val);
            }
        }
        dfs(root, 0, &mut ans);
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    fn bfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            ans.push(vec![]);
            let mut n = queue.len();
            while n > 0 {
                if let Some(Some(r)) = queue.pop_front() {
                    ans.last_mut().unwrap().push(r.as_ref().borrow().val);
                    let left = r.as_ref().borrow().left.clone();
                    if left.is_some() {
                        queue.push_back(left);
                    }
                    let right = r.as_ref().borrow().right.clone();
                    if right.is_some() {
                        queue.push_back(right);
                    }
                }
                n -= 1;
            }
        }
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    bfs_approach(root)
}

// https://leetcode.com/problems/path-sum-iii/description/
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    fn optimized_approach(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            cur_path: &mut Vec<i64>,
            cur_sum: i64,
            target_sum: i64,
            path_count: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.as_ref().borrow();
                cur_path.push(r.val as i64);
                let mut total_sum = cur_sum + r.val as i64;
                if total_sum == target_sum {
                    *path_count += 1;
                }
                for i in 0..cur_path.len() - 1 {
                    total_sum -= cur_path[i];
                    if total_sum == target_sum {
                        *path_count += 1;
                    }
                }
                dfs(
                    r.left.clone(),
                    cur_path,
                    cur_sum + r.val as i64,
                    target_sum,
                    path_count,
                );
                dfs(
                    r.right.clone(),
                    cur_path,
                    cur_sum + r.val as i64,
                    target_sum,
                    path_count,
                );
                cur_path.pop();
            }
        }
        let mut cur_path = vec![];
        let mut path_count = 0;
        dfs(root, &mut cur_path, 0, target_sum as i64, &mut path_count);
        path_count
    }
    fn brute_force_approach(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            cur_sum: i64,
            target_sum: i64,
            path_count: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.as_ref().borrow();
                let sum = cur_sum + r.val as i64;
                if sum == target_sum {
                    *path_count += 1;
                }
                dfs(r.left.clone(), sum, target_sum, path_count);
                dfs(r.right.clone(), sum, target_sum, path_count);
            }
        }
        let mut path_count = 0;
        dfs(root.clone(), 0, target_sum as i64, &mut path_count);
        path_count += brute_force_approach(
            root.clone().and_then(|x| x.as_ref().borrow().left.clone()),
            target_sum,
        );
        path_count += brute_force_approach(
            root.and_then(|x| x.as_ref().borrow().right.clone()),
            target_sum,
        );
        path_count
    }
    brute_force_approach(root, target_sum)
}

// https://leetcode.com/problems/powx-n/
pub fn my_pow(x: f64, n: i32) -> f64 {
    fn rec(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let u = rec(x, n / 2);
        if n % 2 == 0 {
            u * u
        } else {
            u * u * x
        }
    }
    let x = rec(x, n.abs());
    if n < 0 {
        1.0 / x
    } else {
        x
    }
}

// https://leetcode.com/problems/search-a-2d-matrix/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn search_row(matrix: &Vec<Vec<i32>>, target: i32) -> usize {
        let mut lo = 0;
        let mut hi = matrix.len() - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if matrix[mid][matrix[0].len() - 1] >= target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
    fn search_col(matrix: &[i32], target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = matrix.len() as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if matrix[mid as usize] == target {
                return mid as i32;
            } else if matrix[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
    let row_idx = search_row(&matrix, target);
    let col_idx = search_col(&matrix[row_idx], target);
    col_idx >= 0
}

// https://leetcode.com/problems/largest-number/
// https://leetcode.com/problems/largest-number/editorial/
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    nums.sort_by(|a, b| {
        let o1 = format!("{}{}", a, b);
        let o2 = format!("{}{}", b, a);
        o1.cmp(&o2)
    });
    nums.reverse();
    if nums[0] == "0" {
        return "0".to_string();
    }
    nums.join("")
}

// https://leetcode.com/problems/decode-ways/description/
// https://leetcode.com/problems/decode-ways/solutions/30451/evolve-from-recursion-to-dp/
pub fn num_decodings(s: String) -> i32 {
    fn top_down_approach(s: String) -> i32 {
        fn rec(i: usize, s: &Vec<char>, memo: &mut Vec<i32>) -> i32 {
            if i == s.len() {
                return 1;
            }
            if s[i] == '0' {
                return 0;
            }
            if memo[i] == -1 {
                let mut res = 0;
                res += rec(i + 1, s, memo);
                if i < s.len() - 1
                    && (s[i] == '1' || (s[i] == '2' && ('0'..='6').contains(&s[i + 1])))
                {
                    res += rec(i + 2, s, memo);
                }
                memo[i] = res
            }
            memo[i]
        }
        let mut memo = vec![-1; s.len()];
        rec(0, &s.chars().collect(), &mut memo)
    }
    fn bottom_up_approach(s: String) -> i32 {
        let mut dp1 = 1;
        let mut dp2 = 0;
        let mut s = s.chars().collect::<Vec<_>>();
        for i in (0..s.len()).rev() {
            let mut dp = if s[i] == '0' { 0 } else { dp1 };
            if i < s.len() - 1 && (s[i] == '1' || (s[i] == '2' && ('0'..='6').contains(&s[i + 1])))
            {
                dp += dp2;
            }
            dp2 = dp1;
            dp1 = dp;
        }
        dp1
    }
    bottom_up_approach(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", zigzag_level_order(root));
    }

    #[test]
    fn test_path_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", path_sum(root, 8)); // 3
    }

    #[test]
    fn test_my_pow() {
        println!("{}", my_pow(2.00000, 10));
    }

    #[test]
    fn test_search_matrix() {
        println!("{}", search_matrix(vec![vec![1]], 1)); // true
        println!("{}", search_matrix(vec![vec![1], vec![3]], 1)); // true

        println!(
            "{}",
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            )
        ); // true

        println!(
            "{}",
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            )
        ); // false
    }

    #[test]
    fn test_largest_number() {
        println!("{}", largest_number(vec![3, 30, 34, 5, 9])); // 9534330
    }

    #[test]
    fn test_num_decodings() {
        println!("{}", num_decodings("12".to_string())); // 2
        println!("{}", num_decodings("226".to_string())); // 3
        println!("{}", num_decodings("226".to_string())); // 3
        println!("{}", num_decodings("06".to_string())); // 0
        println!("{}", num_decodings("10".to_string())); // 1
        println!("{}", num_decodings("2611055971756562".to_string())); // 4
    }
}
