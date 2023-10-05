// https://leetcode.com/problems/majority-element-ii/
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let (mut count1, mut count2) = (0, 0);
    let (mut candidate1, mut candidate2) = (None, None);

    for &n in &nums {
        if candidate1.is_some() && candidate1.unwrap() == n {
            count1 += 1;
        } else if candidate2.is_some() && candidate2.unwrap() == n {
            count2 += 1;
        } else if count1 == 0 {
            candidate1 = Some(n);
            count1 = 1;
        } else if count2 == 0 {
            candidate2 = Some(n);
            count2 = 1;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
    }

    let mut result = vec![];
    let (mut count1, mut count2) = (0, 0);

    for &n in &nums {
        if candidate1.is_some() && candidate1.unwrap() == n {
            count1 += 1;
        }
        if candidate2.is_some() && candidate2.unwrap() == n {
            count2 += 1;
        }
    }

    let n = nums.len() as i32;
    if count1 > n / 3 {
        result.push(candidate1.unwrap());
    }
    if count2 > n / 3 {
        result.push(candidate2.unwrap());
    }
    result
}

// https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array/description/
pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    fn lower_bound(nums: &[i32], target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;
        let mut index = nums.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] >= target {
                hi = mid;
                index = mid;
            } else {
                lo = mid + 1;
            }
        }
        index
    }

    fn upper_bound(nums: &[i32], target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] <= target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }

    let upper = upper_bound(&nums, target);
    let lower = lower_bound(&nums, target);

    // println!("{} {}", lower, upper);
    upper - lower > nums.len() as i32 / 2
}
