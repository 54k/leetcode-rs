// https://leetcode.com/problems/search-insert-position/description/
pub fn search_insert_i(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0 as i32;
    let mut hi = nums.len() as i32;

    while lo < hi {
        let mid = (lo + hi) / 2;

        if nums[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
}

pub fn search_insert_ii(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0 as i32;
    let mut hi = nums.len() as i32 - 1;

    while lo <= hi {
        let mid = (lo + hi) / 2;

        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    lo
}

// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
pub fn min_operations(nums: Vec<i32>) -> i32 {
    fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if target < nums[mid as usize] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    use std::collections::HashSet;
    let n = nums.len() as i32;
    let mut ans = n as i32;
    let unique = nums.iter().copied().collect::<HashSet<_>>();
    let mut new_nums = vec![0; unique.len()];
    let mut index = 0;

    for &num in &unique {
        new_nums[index] = num;
        index += 1;
    }

    new_nums.sort();

    for i in 0..new_nums.len() {
        let left = new_nums[i];
        let right = left + n - 1;
        let j = binary_search(&new_nums, right);
        let count = j - i as i32;
        ans = ans.min(n - count);
    }

    ans
}
