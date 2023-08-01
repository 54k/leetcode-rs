// https://leetcode.com/problems/minimum-index-of-a-valid-split/description/
pub fn minimum_index(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let f = nums.iter().copied().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });
    let mut f1 = HashMap::new();
    let mut f2 = HashMap::new();

    for i in 0..nums.len() - 1 {
        *f1.entry(nums[i]).or_insert(0) += 1;
        *f2.entry(nums[i]).or_insert(0) = f[&nums[i]] - f1[&nums[i]];

        println!(
            "el {} f1 {:?} power {}, len {}",
            nums[i],
            f1,
            f1[&nums[i]] * 2,
            i + 1
        );
        println!(
            "el {} f2 {:?} power {}, len {}",
            nums[i],
            f2,
            f2[&nums[i]] * 2,
            nums.len() - i - 1
        );

        if f1[&nums[i]] * 2 > i + 1 && f2[&nums[i]] * 2 > nums.len() - i - 1 {
            return i as i32;
        }
    }

    -1
}

// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/
pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut ans = 0;
    let mut left = 0;
    for right in 0..nums.len() {
        while nums[right] - nums[left] > 2 * k {
            left += 1;
        }
        ans = ans.max((right - left + 1) as i32);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_idx() {
        println!("{}", minimum_index(vec![1, 2, 2, 2]));
        println!("{}", minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]));
        println!("{}", minimum_index(vec![3, 3, 3, 3, 7, 2, 2]));
    }
}
