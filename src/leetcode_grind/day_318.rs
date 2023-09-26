// https://leetcode.com/problems/remove-duplicate-letters/description
pub fn remove_duplicate_letters(s: String) -> String {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let s = s.chars().collect::<Vec<_>>();
    let mut last_idx = HashMap::new();
    let mut on_stack = HashSet::new();

    for i in 0..s.len() {
        last_idx.insert(s[i], i);
    }

    let mut stack = vec![];
    for i in 0..s.len() {
        if !on_stack.contains(&s[i]) {
            while stack.len() > 0
                && i < last_idx[&stack[stack.len() - 1]]
                && stack[stack.len() - 1] >= s[i]
            {
                on_stack.remove(&stack.pop().unwrap());
            }
            stack.push(s[i]);
            on_stack.insert(s[i]);
        }
    }

    stack.into_iter().collect::<_>()
}

// https://leetcode.com/problems/3sum-smaller/description/
pub fn three_sum_smaller_bin_search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    fn two_sum_smaller(nums: &Vec<i32>, start: usize, target: i32) -> i32 {
        fn binary_search(nums: &Vec<i32>, start: usize, target: i32) -> usize {
            let mut lo = start;
            let mut hi = nums.len() - 1;

            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if nums[mid] < target {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            lo
        }

        let mut sum = 0;
        for i in start..nums.len() - 1 {
            let j = binary_search(nums, i, target - nums[i]);
            sum += j as i32 - i as i32;
        }
        sum
    }

    let mut nums = nums;
    nums.sort();
    let mut sum = 0;
    for i in 0..nums.len() - 2 {
        sum += two_sum_smaller(&nums, i + 1, target - nums[i]);
    }
    sum
}

pub fn three_sum_smaller_two_pointers(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    fn two_sum_smaller(nums: &Vec<i32>, start: usize, target: i32) -> i32 {
        let mut sum = 0;
        let mut left = start;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] + nums[right] < target {
                sum += right as i32 - left as i32;
                left += 1;
            } else {
                right -= 1;
            }
        }
        sum
    }

    let mut nums = nums;
    nums.sort();
    let mut count = 0;

    for i in 0..nums.len() - 2 {
        count += two_sum_smaller(&nums, i + 1, target - nums[i]);
    }

    count
}
