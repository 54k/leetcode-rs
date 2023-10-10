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

// https://leetcode.com/problems/valid-number/description/
pub fn is_number(s: String) -> bool {
    use std::collections::{HashMap, HashSet};
    let dfa = vec![
        HashMap::from([("num", 1), ("sign", 2), ("dot", 3)]),
        HashMap::from([("num", 1), ("dot", 4), ("exponent", 5)]),
        HashMap::from([("num", 1), ("dot", 3)]),
        HashMap::from([("num", 4)]),
        HashMap::from([("num", 4), ("exponent", 5)]),
        HashMap::from([("sign", 6), ("num", 7)]),
        HashMap::from([("num", 7)]),
        HashMap::from([("num", 7)]),
    ];

    let finite_state = HashSet::from([1, 4, 7]);

    let mut curr_state = 0;
    let mut curr_group = "";

    let s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        if char::is_digit(s[i], 10) {
            curr_group = "num";
        } else if s[i] == '-' || s[i] == '+' {
            curr_group = "sign";
        } else if s[i] == 'e' || s[i] == 'E' {
            curr_group = "exponent";
        } else if s[i] == '.' {
            curr_group = "dot";
        } else {
            return false;
        }

        if dfa[curr_state].contains_key(curr_group) {
            curr_state = dfa[curr_state][curr_group];
        } else {
            return false;
        }
    }

    finite_state.contains(&curr_state)
}

#[test]
fn test_is_number() {
    let res = is_number("2e0".to_string());
    println!("{res}");
}
