#[allow(dead_code)]
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut m = HashMap::new();
    let mut s = HashSet::new();

    for x in arr.iter() {
        m.entry(x).or_insert(1);
        *m.get_mut(&x).unwrap() += 1;
    }
    for v in m.values() {
        s.insert(v);
    }
    m.keys().len() == s.len()
}

// https://leetcode.com/problems/count-subarrays-with-median-k/discuss/2851940/Balance
#[allow(dead_code)]
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut res = 0;
    let mut p = 0;

    // find k index
    for (i, e) in nums.iter().enumerate() {
        if *e == k {
            p = i;
            break;
        }
    }

    let mut balances = HashMap::new();

    // right side, tracking balance
    let mut right_bal = 0;
    for i in p..nums.len() {
        if i == p {
            right_bal += 0;
        } else if nums[i] < nums[p] {
            right_bal -= 1;
        } else {
            right_bal += 1;
        }
        balances.entry(right_bal).or_insert(0);
        *balances.get_mut(&right_bal).unwrap() += 1;
    }

    // left side, tracking balance
    let mut left_bal = 0;
    for i in (0..=p).rev() {
        if i == p {
            left_bal += 0;
        } else if nums[i] < nums[p] {
            left_bal -= 1;
        } else {
            left_bal += 1;
        }
        // find complementary right balance, balance == 0 for even sub-arrays, balance == 1 for odd sub-arrays
        res +=
            *balances.get(&-left_bal).unwrap_or(&0) + balances.get(&(-left_bal + 1)).unwrap_or(&0);
    }

    res
}

#[cfg(test)]
mod test {
    use crate::day_18::*;

    #[test]
    fn test89() {
        println!("{}", unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn test90() {
        println!("{}", count_subarrays(vec![7, 1, 3, 4, 2, 5, 6], 4)); // 5
    }
}
