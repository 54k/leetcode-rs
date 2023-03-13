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
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    res
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
}
