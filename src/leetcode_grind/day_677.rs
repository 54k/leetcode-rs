// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/description/?envType=daily-question&envId=2024-09-24
pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut all_prefixes = HashSet::new();
    for mut val in arr1 {
        while !all_prefixes.contains(&val) && val > 0 {
            all_prefixes.insert(val);
            val /= 10;
        }
    }
    let mut longest_prefix = 0;
    for mut val in arr2 {
        while !all_prefixes.contains(&val) && val > 0 {
            val /= 10;
        }
        if val > 0 {
            longest_prefix = longest_prefix.max(format!("{val}").len() as i32);
        }
    }
    longest_prefix
}
