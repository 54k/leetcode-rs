use crate::day_58::recover_tree;
use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/search-in-rotated-sorted-array/description/
// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/14425/concise-o-log-n-binary-search-solution/
// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/14419/pretty-short-c-java-ruby-python/?orderBy=most_relevant
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn two_bin_search(nums: Vec<i32>, target: i32) -> i32 {
        fn find_pivot(nums: &Vec<i32>) -> usize {
            let mut lo = 0;
            let mut hi = nums.len() - 1;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] > nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }
        fn bin_search(nums: &Vec<i32>, target: i32, rot: usize) -> i32 {
            let mut lo = 0_i32;
            let mut hi = nums.len() as i32 - 1;

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let real_mid = (mid as usize + rot) % nums.len();
                if nums[real_mid] < target {
                    lo = mid + 1;
                } else if nums[real_mid] > target {
                    hi = mid - 1;
                } else {
                    return real_mid as i32;
                }
            }
            -1
        }
        let rot = find_pivot(&nums);
        bin_search(&nums, target, rot)
    }
    two_bin_search(nums, target)
}

// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/description/
// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/editorial/
pub fn search_ii(nums: Vec<i32>, target: i32) -> bool {
    fn is_bin_search_helpful(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        nums[start] != element
    }
    fn exists_in_first_array(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        // returns true if element exists in first array, false if it exists in second
        nums[start] <= element
    }
    if nums.is_empty() {
        return false;
    }
    let n = nums.len();
    let mut lo = 0;
    let mut hi = n - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] == target {
            return true;
        }
        if !is_bin_search_helpful(&nums, lo, nums[mid]) {
            lo += 1;
            continue;
        }
        // which array does the pivot belongs to
        let pivot_array = exists_in_first_array(&nums, lo, nums[mid]);
        // which array does the target belongs to
        let target_array = exists_in_first_array(&nums, lo, target);

        if pivot_array ^ target_array {
            // If pivot and target exist in different sorted arrays,
            // recall that xor is true when both operands are distinct
            if pivot_array {
                lo = mid + 1;
                // pivot in the first, target in the second
            } else {
                hi = mid - 1;
                // target in the first, pivot in the second
            }
        } else {
            // if pivot and target exist in same sorted array
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    false
}

// https://leetcode.com/problems/combination-sum/description/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn rec(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            combinations.push(current_combination.clone());
            return;
        }
        for i in start..candidates.len() {
            current_combination.push(candidates[i]);
            rec(
                candidates,
                target - candidates[i],
                i,
                current_combination,
                combinations,
            );
            current_combination.pop();
        }
    }
    let mut result = vec![];
    rec(&candidates, target, 0, &mut vec![], &mut result);
    result
}

// https://leetcode.com/problems/combination-sum-ii/description/
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn rec(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            combinations.push(current_combination.clone());
            return;
        }

        for i in start..candidates.len() {
            let picked_num = candidates[i];
            if i > start && picked_num == candidates[i - 1] {
                continue;
            }
            current_combination.push(picked_num);
            rec(
                candidates,
                target - picked_num,
                i + 1,
                current_combination,
                combinations,
            );
            current_combination.pop();
        }
    }

    candidates.sort();
    let mut result = vec![];
    rec(&candidates, target, 0, &mut vec![], &mut result);
    result
}

// https://leetcode.com/problems/permutations/
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn permute_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(
            nums: &Vec<i32>,
            mut occupied: i32,
            current_permutation: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current_permutation.len() == nums.len() {
                result.push(current_permutation.clone());
                return;
            }
            for i in 0..nums.len() {
                if (occupied & (1 << i)) == 0 {
                    occupied |= 1 << i;
                    current_permutation.push(nums[i]);
                    rec(nums, occupied, current_permutation, result);
                    current_permutation.pop();
                    occupied ^= 1 << i;
                }
            }
        }
        let mut result = vec![];
        rec(&nums, 0, &mut vec![], &mut result);
        result
    }
    permute_recursive(nums)
}

// https://leetcode.com/problems/merge-intervals/
// https://leetcode.com/problems/merge-intervals/editorial/
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn emre_solution(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut ans = vec![];
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= end {
                end = end.max(interval[1]);
            } else {
                ans.push(vec![start, end]);
                start = interval[0];
                end = interval[1];
            }
        }
        ans.push(vec![start, end]);
        ans
    }

    fn leetcode_sort(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut merged: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            // if the list of merged intervals is empty or if the current
            // interval does not overlap with the previous, simply append it.
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                merged.push(interval);
            } else {
                // otherwise, there is overlap, so we merge the current and previous intervals.
                merged.last_mut().unwrap()[1] = merged.last().unwrap()[1].max(interval[1]);
            }
        }
        merged
    }
    leetcode_sort(intervals)
}

// https://leetcode.com/problems/teemo-attacking/description/
// https://leetcode.com/problems/teemo-attacking/editorial/
pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut ans = 0;
    for i in 0..time_series.len() {
        if i < time_series.len() - 1 {
            ans += duration.min(time_series[i + 1] - time_series[i]);
        } else {
            ans += duration;
        }
    }
    ans
}

// https://leetcode.com/problems/dota2-senate/description/
pub fn predict_party_victory(senate: String) -> String {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut radiant_banned = 0;
    let mut dire_banned = 0;
    let mut radiant_in_queue = 0;
    let mut dire_in_queue = 0;

    for senator in senate.chars() {
        if senator == 'R' {
            radiant_in_queue += 1;
        } else {
            dire_in_queue += 1;
        }
        queue.push_back(senator);
    }

    while radiant_in_queue > 0 && dire_in_queue > 0 {
        if let Some('R') = queue.pop_front() {
            if radiant_banned > 0 {
                radiant_banned -= 1;
                radiant_in_queue -= 1;
                continue;
            }
            dire_banned += 1;
            queue.push_back('R');
        } else {
            if dire_banned > 0 {
                dire_banned -= 1;
                dire_in_queue -= 1;
                continue;
            }
            radiant_banned += 1;
            queue.push_back('D');
        }
    }

    if let Some('R') = queue.front() {
        "Radiant".to_string()
    } else {
        "Dire".to_string()
    }
}

// https://leetcode.com/problems/2-keys-keyboard/description/
// https://leetcode.com/problems/2-keys-keyboard/editorial/
pub fn min_steps(mut n: i32) -> i32 {
    fn leetcode_min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 2;
        while n > 1 {
            while n % i == 0 {
                ans += i;
                n /= i;
            }
            i += 1;
        }
        ans
    }
    fn laakonsen_cp_min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 2;
        while i * i <= n {
            while n % i == 0 {
                ans += i;
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            ans += n;
        }
        ans
    }
    laakonsen_cp_min_steps(n)
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn lowest_common_ancestor_postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let r = root.clone()?;
        let r = r.borrow();
        let lcp_left = lowest_common_ancestor_postorder(r.left.clone(), p.clone(), q.clone());
        let lcp_right = lowest_common_ancestor_postorder(r.right.clone(), p, q);
        if lcp_left.is_some() && lcp_right.is_some() {
            root
        } else if lcp_left.is_some() {
            lcp_left
        } else {
            lcp_right
        }
    }
    lowest_common_ancestor_postorder(root, p, q)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 0)); // 4
        println!("{}", search(vec![1, 3], 1)); // 0
        println!("{}", search(vec![3, 1], 1)); // 1
        println!("{}", search(vec![3, 1], 3)); // 0

        println!("{}", search(vec![3, 5, 1], 1)); // 2
        println!("{}", search(vec![3, 5, 1], 3)); // 0
        println!("{}", search(vec![5, 1, 3], 1)); // 1
        println!("{}", search(vec![5, 1, 3], 5)); // 0
        println!("{}", search(vec![5, 1, 3], 3)); // 2

        println!("{}", search_ii(vec![2, 2, 2, 3, 2, 2, 2], 3)); // true
        println!("{}", search_ii(vec![2, 5, 6, 0, 0, 1, 2], 0)); // true
        println!(
            "{}",
            search_ii(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            )
        ); // true
    }

    #[test]
    fn test_combination_sum() {
        println!("{:?}", combination_sum(vec![2, 3, 6, 7], 7)); // [[2,2,3],[7]]
        println!("{:?}", combination_sum(vec![2, 3, 5], 8)); // [[2,2,2,2],[2,3,3],[3,5]]
        println!("{:?}", combination_sum(vec![2], 1)); // [[2,2,2,2],[2,3,3],[3,5]]

        println!("{:?}", combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)); // [[1,1,6],[1,2,5],[1,7],[2,6]]
        println!("{:?}", combination_sum2(vec![2, 5, 2, 1, 2], 5)); // [[1,2,2],[5]]
    }

    #[test]
    fn test_permute() {
        println!("{:?}", permute(vec![1, 2, 3]));
    }

    #[test]
    fn test_merge() {
        println!(
            "{:?}",
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        ); // [[1,6],[8,10],[15,18]]
    }

    #[test]
    fn test_find_poisoned_duration() {
        println!("{}", find_poisoned_duration(vec![1, 4], 2)); // 4
        println!("{}", find_poisoned_duration(vec![1, 2], 2)); // 3
    }

    #[test]
    fn test_predict_party_victory() {
        println!("{}", predict_party_victory("RD".to_string())); // Radiant
        println!("{}", predict_party_victory("DR".to_string())); // Dire
        println!("{}", predict_party_victory("RDD".to_string())); // Dire
        println!("{}", predict_party_victory("DDRRR".to_string())); // Dire
    }

    #[test]
    fn test_min_steps() {
        println!("{}", min_steps(9));

        println!("{}", min_steps(3));
        println!("{}", min_steps(10));
        println!("{}", min_steps(17));
    }

    #[test]
    fn test_lowest_common_ancestor() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: p.clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: q.clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", lowest_common_ancestor(root, p, q)); // 1
    }
}
