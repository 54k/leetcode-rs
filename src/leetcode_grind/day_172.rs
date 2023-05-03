// https://leetcode.com/problems/find-the-difference-of-two-arrays/description/
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let s1 = nums1.into_iter().collect::<HashSet<i32>>();
    let s2 = nums2.into_iter().collect::<HashSet<i32>>();
    let mut ans = vec![vec![]; 2];
    for e in &s1 {
        if !s2.contains(e) {
            ans[0].push(*e);
        }
    }
    for e in &s2 {
        if !s1.contains(e) {
            ans[1].push(*e);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test480() {
        println!("{:?}", find_difference(vec![1, 2, 3], vec![2, 4, 6])); // 1,3 / 4,6
    }
}
