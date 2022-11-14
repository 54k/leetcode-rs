#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::{max, min, Ordering};
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut result = 0;
    while left < right {
        let w = (right - left) as i32;
        let h = min(height[left], height[right]);
        let area = w * h;

        result = max(result, area);

        match height[left].cmp(&height[right]) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => {
                left += 1;
                right -= 1;
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left]
            } else {
                ans += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }
    ans
}

#[allow(dead_code)]
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    nums.sort();
    let mut ans = vec![];

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            match sum.cmp(&0) {
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Less => {
                    left += 1;
                }
                _ => {
                    ans.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while right > left && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
    }
    ans
}

#[allow(dead_code)]
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = nums[0] + nums[1] + nums[nums.len() - 1];
    nums.sort();
    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > target {
                right -= 1;
            } else {
                left += 1;
            }

            if (sum - target).abs() < (ans - target).abs() {
                ans = sum;
            }
        }
    }
    ans
}

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut set = HashMap::<i32, i32>::new();
    for (i, it) in nums.iter().enumerate() {
        let f = target - it;
        if set.contains_key(&f) {
            return vec![i as i32, *set.get(&f).unwrap()];
        }
        set.insert(nums[i], i as i32);
    }
    vec![]
}

#[allow(dead_code)]
pub fn two_sum_2(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;
    nums.sort();

    let mut ans = vec![];
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => {
                ans.push(left as i32 + 1);
                ans.push(right as i32 + 1);
                left += 1;
                right -= 1;
            }
        }
    }
    ans
}

#[allow(dead_code)]
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn two_sum(nums: &Vec<i32>, target: i128, start: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut lo = start;
        let mut hi = (nums.len() - 1) as i32;
        while lo < hi {
            let sum = nums[lo as usize] as i128 + nums[hi as usize] as i128;
            if sum < target || (lo > start && nums[lo as usize] == nums[lo as usize - 1]) {
                lo += 1;
            } else if sum > target
                || (hi < (nums.len() - 1) as i32 && nums[hi as usize] == nums[hi as usize + 1])
            {
                hi -= 1;
            } else {
                res.push(vec![nums[lo as usize], nums[hi as usize]]);
                lo += 1;
                hi -= 1;
            }
        }
        res
    }

    fn k_sum(nums: &Vec<i32>, target: i128, start: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if start == nums.len() as i32 {
            return res;
        }
        let avg_value: i128 = target / k as i128;

        if nums[start as usize] as i128 > avg_value || avg_value > nums[nums.len() - 1] as i128 {
            return res;
        }

        if k == 2 {
            return two_sum(nums, target, start);
        }

        for i in start as usize..nums.len() {
            if i as i32 == start || nums[i - 1] != nums[i] {
                for subset in k_sum(nums, target - nums[i] as i128, i as i32 + 1, k - 1) {
                    let mut v = vec![nums[i]];
                    v.extend(subset.iter());
                    res.push(v);
                }
            }
        }
        res
    }

    nums.sort();
    k_sum(&nums, target as i128, 0, 4)
}

#[allow(dead_code)]
pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut res = 0;
    let mut pair_count_by_sum = HashMap::<i32, i32>::new();

    for i in &nums1 {
        for j in &nums2 {
            let sum = i + j;
            if pair_count_by_sum.contains_key(&sum) {
                let c = pair_count_by_sum.remove(&sum).unwrap();
                pair_count_by_sum.insert(sum, c + 1);
            } else {
                pair_count_by_sum.insert(sum, 1);
            }
        }
    }

    for i in &nums3 {
        for j in &nums4 {
            let sum = -(i + j);
            if pair_count_by_sum.contains_key(&sum) {
                res += pair_count_by_sum.get(&sum).unwrap();
            }
        }
    }

    res
}

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let literal_map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut res = 0;
    let chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        let mut num = *literal_map.get(&ch).unwrap();
        if i < chars.len() - 1 {
            let next_num = *literal_map.get(&chars[i + 1]).unwrap();
            if next_num > num {
                num = next_num - num;
                i += 1;
            }
        }
        res += num;
        i += 1;
    }
    res
}

#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len = nums1.len() + nums2.len();
    let mut merged = Vec::with_capacity(len);

    let mut i = 0;
    let mut j = 0;

    for _ in 0..len {
        if j >= nums2.len() {
            merged.push(nums1[i]);
            i += 1;
        } else if i >= nums1.len() {
            merged.push(nums2[j]);
            j += 1;
        } else if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    if len % 2 == 0 {
        return (merged[len / 2 - 1] + merged[len / 2]) as f64 / 2.;
    }
    merged[len / 2] as f64
}

#[cfg(test)]
mod test {
    use crate::day_2::*;

    #[test]
    fn test14() {
        println!("{:?}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn test15() {
        println!("{:?}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn test16() {
        println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
        println!("{:?}", three_sum(vec![0, 0, 0]));
        println!("{:?}", three_sum(vec![-2, 0, 0, 2, 2]));
    }

    #[test]
    fn test17() {
        println!("{:?}", three_sum_closest(vec![-1, 2, 1, -4], 2));
    }

    #[test]
    fn test18() {
        println!("{:?}", two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test19() {
        println!("{:?}", two_sum_2(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test20() {
        println!(
            "{:?}",
            four_sum(
                vec![
                    -1000000000,
                    -1000000000,
                    1000000000,
                    -1000000000,
                    -1000000000
                ],
                294967296
            )
        );
        println!("{:?}", four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    }

    #[test]
    fn test21() {
        println!(
            "{:?}",
            four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2])
        );
        println!(
            "{:?}",
            four_sum_count(
                vec![-1, 1, 1, 1, -1],
                vec![0, -1, -1, 0, 1],
                vec![-1, -1, 1, -1, -1],
                vec![0, 1, 0, -1, -1]
            )
        );
    }

    #[test]
    fn test22() {
        println!("{:?}", roman_to_int("MCMXCIV".to_owned()));
    }

    #[test]
    fn test23() {
        println!("{:?}", find_median_sorted_arrays(vec![1, 3], vec![2]));
    }
}
