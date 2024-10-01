// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/description/?envType=daily-question&envId=2024-10-01
pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    for &num in &arr {
        let k = ((num % k) + k) % k;
        *cnt.entry(k).or_insert(0) += 1;
    }
    for num in arr {
        let i = ((num % k) + k) % k;
        if i == 0 {
            if cnt.get(&i).unwrap() % 2 == 1 {
                return false;
            }
        } else if cnt.get(&i) != cnt.get(&(k - i)) {
            return false;
        }
    }
    true
}
