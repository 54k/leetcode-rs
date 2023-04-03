use crate::leetcode_grind::day_2::roman_to_int;

// https://leetcode.com/problems/boats-to-save-people/
// https://leetcode.com/problems/boats-to-save-people/editorial/
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut ans = 0;
    let mut i = 0;
    let mut j = people.len() as i32 - 1;
    while i <= j {
        ans += 1;
        if people[i as usize] + people[j as usize] <= limit {
            i += 1;
        }
        j -= 1;
    }
    ans
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut ans = vec![0; nums.len()];
    for k in (0..nums.len()).rev() {
        let square = if nums[i].abs() < nums[j].abs() {
            j -= 1;
            nums[j + 1]
        } else {
            i += 1;
            nums[i - 1]
        };
        ans[k] = square * square;
    }
    ans
}

// https://leetcode.com/problems/subarray-product-less-than-k/description/
// https://leetcode.com/problems/subarray-product-less-than-k/editorial/
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut current_prod = 1;
    let mut left = 0_i32;
    for right in 0..nums.len() as i32 {
        current_prod *= nums[right as usize];
        while left <= right && current_prod >= k {
            current_prod /= nums[left as usize];
            left += 1;
        }
        ans += right - left + 1;
    }
    ans
}

// https://leetcode.com/problems/maximum-average-subarray-i/description/
// https://leetcode.com/problems/maximum-average-subarray-i/editorial/
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut cur = 0;
    let mut left = 0;
    let mut ans = 0;
    for right in 0..nums.len() {
        if right >= k as usize {
            ans = ans.max(cur);
            cur -= nums[left];
            left += 1;
        }
        cur += nums[right];
    }
    ans = ans.max(cur);
    ans as f64 / k as f64
}

// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/
// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/
pub fn min_start_value(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test391() {
        println!("{}", num_rescue_boats(vec![3, 5, 3, 4], 5)); // 4
    }

    #[test]
    fn test392() {
        println!("{:?}", sorted_squares(vec![-4, -1, 0, 3, 10]));
    }

    #[test]
    fn test393() {
        println!(
            "{:?}",
            num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );
    }

    #[test]
    fn test394() {
        // println!("{:?}", find_max_average(vec![1, 12, -5, -6, 50, 3], 4)); // 12.75
        println!("{:?}", find_max_average(vec![5], 1)); // 12.75
    }

    #[test]
    fn test395() {
        println!("{}", min_start_value(vec![-3, 2, -3, 4, 2])); // 6
    }
}
