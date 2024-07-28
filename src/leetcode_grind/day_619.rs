// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut chars = vec![None; 128];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    while right < s.len() {
        let r = s[right];
        let idx = chars[r as usize];

        if let Some(i) = idx {
            if i >= left && i < right {
                left = i + 1;
            }
        }

        res = res.max(right - left + 1);
        chars[r as usize] = Some(right);
        right += 1;
    }
    res as i32
}

// https://leetcode.com/problems/container-with-most-water/description/
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let width = right - left;
        max_area = max_area.max(height[left].min(height[right]) * width as i32);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}

// https://leetcode.com/problems/3sum/description/
pub fn three_sum_i(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn two_sum(nums: &Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        let mut j = i + 1;
        while j < nums.len() {
            let complement = -nums[i] - nums[j];
            if seen.contains(&complement) {
                res.push(vec![nums[i], nums[j], complement]);
                while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                    j += 1;
                }
            }
            seen.insert(nums[j]);
            j += 1;
        }
    }

    let mut nums = nums;
    nums.sort();
    let mut res = vec![];

    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }

        if i == 0 || nums[i - 1] != nums[i] {
            two_sum(&nums, i, &mut res);
        }
    }

    res
}

pub fn three_sum_ii(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, HashSet};
    let mut res = HashSet::new();
    let mut dups = HashSet::new();
    let mut seen = HashMap::new();

    for i in 0..nums.len() {
        if dups.insert(nums[i]) {
            for j in i + 1..nums.len() {
                let complement = -nums[i] - nums[j];
                if seen.contains_key(&complement) && *seen.get(&complement).unwrap() == i {
                    let mut triplet = vec![nums[i], nums[j], complement];
                    triplet.sort();
                    res.insert(triplet);
                }
                seen.insert(nums[j], i);
            }
        }
    }
    res.into_iter().collect()
}

// https://leetcode.com/problems/next-permutation/description/
pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut nums = nums;
    let mut pivot = n - 1;

    while pivot >= 1 && nums[pivot] <= nums[pivot - 1] {
        pivot -= 1;
    }

    if pivot != 0 {
        let mut j = n - 1;
        while nums[j] <= nums[pivot - 1] {
            j -= 1;
        }
        nums.swap(pivot - 1, j);
    }

    nums[pivot..n].reverse();
}
