#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut min: i32, mut max: i32) -> i32 {
        if root.is_none() {
            return max - min;
        }
        let root = root.clone().unwrap();
        let root = root.borrow();

        min = min.min(root.val);
        max = max.max(root.val);

        let left = dfs(root.left.clone(), min, max);
        let right = dfs(root.right.clone(), min, max);

        return left.max(right);
    }

    if root.is_none() {
        return 0;
    }

    let x = root.clone().unwrap();
    let x = x.borrow();
    dfs(root, x.val, x.val)
}

#[allow(dead_code)]
pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
    use std::collections::HashSet;
    let mut prefix = vec![0; nums.len() + 1];
    for i in 1..=nums.len() {
        if nums[i - 1] % p == 0 {
            prefix[i] = prefix[i - 1] + 1;
        } else {
            prefix[i] = prefix[i - 1];
        }
    }

    let mut set = HashSet::new();
    for i in 0..nums.len() {
        for j in i + 1..=nums.len() {
            if prefix[j] - prefix[i] <= k {
                set.insert(&nums[i..j]);
            }
        }
    }

    return set.len() as i32;
}

#[allow(dead_code)]
pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    fn brute(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let k = k as usize;
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let sa = &nums[i..=j];
                let set = sa.iter().copied().collect::<HashSet<i32>>();
                if set.len() == k {
                    ans += 1;
                }
            }
        }
        ans
    }

    //https://leetcode.com/problems/subarrays-with-k-different-integers/solutions/234542/python-two-pointer-one-sliding-window-o-n-solution/?orderBy=most_relevant
    fn two_pointers(nums: Vec<i32>, k: i32) -> i32 {
        // The idea is to keep the invariant that the sliding window contains K different integers,
        // shrink the sliding window form left as much as we can, and keep track of the number of subarrays ending at each position.
        // The time complexity is O(N), and the space complexity is O(K).
        // Thank @h11129 for pointing out the redundancy in my original version! The modified version is as follows.
        use std::collections::HashMap;

        let k = k as usize;
        let mut ans = 0;
        let mut dp = 0;
        let mut map = HashMap::new();

        let mut i = 0;
        for n in nums.iter() {
            *map.entry(n).or_insert(0) += 1;

            if map.len() < k {
                continue;
            } else if map.len() == k {
                dp = 1.max(dp);
            } else {
                let g = map.get_mut(&nums[i]).unwrap();
                *g -= 1;
                if *g == 0 {
                    map.remove(&nums[i]);
                }
                i += 1;
                dp = 1;
            }

            while *map.get(&nums[i]).unwrap() != 1 {
                dp += 1;
                let g = map.get_mut(&nums[i]).unwrap();
                *g -= 1;
                i += 1;
            }
            ans += dp;
        }
        ans
    }

    two_pointers(nums, k)
}

#[allow(dead_code)]
pub fn can_jump(nums: Vec<i32>) -> bool {
    fn noob(nums: Vec<i32>) -> bool {
        // state: dp[i] stores max next idx
        // formulae: dp[i] = dp[i - 1] >= i ? (nums[i] + i).max(dp[i - 1]) : 0;
        let mut prev = nums[0];
        let mut cur = prev;
        for i in 1..nums.len() {
            cur = if prev >= i as i32 {
                (nums[i] + i as i32).max(prev)
            } else {
                return false;
            };
            prev = cur;
        }
        cur >= (nums.len() - 1) as i32
    }

    fn pro(nums: Vec<i32>) -> bool {
        let mut n = 1;
        for i in (0..nums.len() - 1).rev() {
            n = if nums[i] < n { n + 1 } else { 1 };
        }
        n == 1
    }

    fn pro2(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= goal {
                goal = i
            }
        }
        goal == 0
    }

    noob(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test104() {
        println!("{}", count_distinct(vec![2, 3, 3, 2, 2], 2, 2));
        println!("{}", count_distinct(vec![1, 2, 3, 4], 4, 1));
        println!("{}", count_distinct(vec![6, 20, 5, 18], 3, 14));
    }

    #[test]
    fn test105() {
        println!("{}", subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)); // 7
        println!("{}", subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3)); // 3
    }

    #[test]
    fn test106() {
        println!("{}", can_jump(vec![2, 0, 0])); // true
        println!("{}", can_jump(vec![2, 3, 1, 1, 4])); // true
        println!("{}", can_jump(vec![3, 2, 1, 0, 4])); // false
    }
}
