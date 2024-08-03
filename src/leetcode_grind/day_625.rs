// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for t in target {
        *map.entry(t).or_insert(0) += 1;
    }
    for a in arr {
        *map.entry(a).or_insert(1) -= 1;
        if map[&a] == 0 {
            map.remove(&a);
        }
    }
    map.len() == 0
}
