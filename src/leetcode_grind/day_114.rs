use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/kth-missing-positive-number/description/
pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut next = 1;
    let mut missing_count = 0;
    let mut i = 0;
    while missing_count < k {
        if i < arr.len() && arr[i] == next {
            i += 1;
        } else {
            missing_count += 1;
        }
        next += 1;
    }
    next - 1
}

// https://leetcode.com/problems/pass-the-pillow/
pub fn pass_the_pillow(n: i32, mut time: i32) -> i32 {
    // https://leetcode.com/problems/pass-the-pillow/solutions/3260484/just-a-runnable-solution/
    // n - (n - 1 - time % (n * 2 - 2)).abs()
    let mut dir = 1;
    let mut idx = 1;
    while time > 0 {
        idx += dir;
        time -= 1;
        if idx == n || idx == 1 {
            dir *= -1;
        }
    }
    idx
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/description/
pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, sums: &mut Vec<i64>) {
        if let Some(r) = root {
            if depth == sums.len() {
                sums.push(0);
            }
            let r = r.borrow();
            sums[depth] += r.val as i64;
            dfs(r.left.clone(), depth + 1, sums);
            dfs(r.right.clone(), depth + 1, sums);
        }
    }
    let mut sums = vec![];
    dfs(root, 0, &mut sums);
    sums.sort();
    sums.reverse();
    let k = k as usize - 1;
    if sums.len() > k {
        sums[k]
    } else {
        -1
    }
}

// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/
pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, sums: &mut Vec<(i64, usize)>) {
        if let Some(r) = root {
            if depth == sums.len() {
                sums.push((0, depth + 1));
            }
            let r = r.borrow();
            sums[depth].0 += r.val as i64;
            dfs(r.left.clone(), depth + 1, sums);
            dfs(r.right.clone(), depth + 1, sums);
        }
    }
    let mut sums = vec![];
    dfs(root, 0, &mut sums);
    sums.sort_by(|(x, y), (a, b)| if x == a { b.cmp(y) } else { x.cmp(a) });
    sums[sums.len() - 1].1 as i32
}

// https://leetcode.com/problems/number-of-ways-to-earn-points/
// https://leetcode.com/problems/number-of-ways-to-earn-points/solutions/3258120/java-c-python-knapsack-dp/?orderBy=most_relevant
pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
    // Let ways[i][points] be the number of ways to score a given number
    // of points after solving some questions of the first i types.
    // ways[i][points] is equal to the sum of ways[i-1][points - solved * marks[i] over 0 <= solved <= count_i
    fn classic_knapsack(target: i32, types: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        let mut dp = vec![vec![0; target as usize + 1]; types.len() + 1];
        dp[0][0] = 1;
        for (i, t) in types.iter().enumerate() {
            for t_sum in 0..target as usize + 1 {
                dp[i + 1][t_sum] = dp[i][t_sum];
                for c in 1..=t[0] {
                    let total_score = (t[1] * c) as usize;
                    if t_sum >= total_score {
                        dp[i + 1][t_sum] = (dp[i + 1][t_sum] + dp[i][t_sum - total_score]) % MOD;
                    }
                }
            }
        }
        dp[types.len()][target as usize]
    }
    fn optimized_space(target: i32, types: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for t in types {
            for t_sum in (0..target + 1).rev() {
                for count in 1..=t[0] {
                    if t_sum - (t[1] * count) < 0 {
                        break;
                    }
                    dp[t_sum as usize] =
                        (dp[t_sum as usize] + dp[(t_sum - t[1] * count) as usize]) % MOD
                }
            }
        }
        dp[target as usize]
    }
    assert_eq!(
        classic_knapsack(target, types.clone()),
        optimized_space(target, types.clone())
    );
    optimized_space(target, types)
}

// https://leetcode.com/problems/split-the-array-to-make-coprime-products/description/
// https://leetcode.com/problems/split-the-array-to-make-coprime-products/solutions/3263371/easiest-approach-commented-prime-factors-mapping/
pub fn find_valid_split(nums: Vec<i32>) -> i32 {
    fn leetcode(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut pos = HashMap::new(); // stores last appearance of prime factors
        let mut pf = vec![vec![]; nums.len()]; // stores all prime factors of each index
        for i in 0..nums.len() {
            let mut x = nums[i];
            let mut j = 2;
            while j * j <= x {
                if x % j == 0 {
                    pos.insert(j, i); // updating the last appearance of prime factor j to i
                    pf[i].push(j);
                    while x % j == 0 {
                        x /= j;
                    }
                }
                j += 1;
            }
            if x > 1 {
                // what if x is still a prime number, we do the same for x
                pos.insert(x, i);
                pf[i].push(x);
            }
        }
        let mut mx = -1; // maximum index for of any prime factor till now
        for i in 0..nums.len() - 1 {
            for &j in &pf[i] {
                // updating the maximum position of all prime factors till index i
                mx = mx.max(pos[&j] as i32);
            }
            if mx == i as i32 {
                return i as i32;
            }
        }
        -1
    }
    leetcode(nums)
}

// https://leetcode.com/problems/count-number-of-nice-subarrays/description/
// https://leetcode.com/problems/count-number-of-nice-subarrays/solutions/419319/o-n-solution/
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    fn tricky_method(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut groups = vec![];
        groups.push(1);
        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                groups.push(1);
            } else {
                let last = groups.len() - 1;
                groups[last] += 1;
            }
        }
        for i in k as usize..groups.len() {
            count += groups[i - k as usize] * groups[i];
        }
        count
    }
    fn deque_method(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(-1);
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                queue.push_back(i as i32);
            }
            if queue.len() > k as usize + 1 {
                queue.pop_front();
            }
            if queue.len() == k as usize + 1 {
                ans += queue[1] - queue[0]
            }
        }
        ans
    }
    deque_method(nums, k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test322() {
        println!("{}", find_kth_positive(vec![2, 3, 4, 7, 11], 5)); // 9
        println!("{}", find_kth_positive(vec![1, 2, 3, 4], 2)); // 6
        println!("{}", find_kth_positive(vec![2, 3, 4], 1)); // 1
    }

    #[test]
    fn test323() {
        println!("{}", pass_the_pillow(4, 5)); // 2
        println!("{}", pass_the_pillow(3, 2)); // 3
    }

    #[test]
    fn test324() {
        println!(
            "{}",
            ways_to_reach_target(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]])
        ); // 7
        println!(
            "{}",
            ways_to_reach_target(5, vec![vec![50, 1], vec![50, 2], vec![50, 5]])
        ); // 4
    }

    #[test]
    fn test325() {
        println!("{}", find_valid_split(vec![4, 7, 8, 15, 3, 5])); // 2
        println!("{}", find_valid_split(vec![4, 7, 15, 8, 3, 5])); // -1
    }

    #[test]
    fn test326() {
        println!(
            "{}",
            number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
        ); // 16
        println!("{}", number_of_subarrays(vec![2, 4, 6], 1)); // 0
        println!("{}", number_of_subarrays(vec![1, 1, 2, 1, 1], 3)); // 2
    }
}
