// https://leetcode.com/problems/falling-squares/
// https://leetcode.com/problems/falling-squares/editorial/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

// https://leetcode.com/problems/maximum-subsequence-score/description/
pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let k = k as usize;
    let mut arr = vec![];
    for i in 0..nums1.len() {
        arr.push((nums1[i], nums2[i]));
    }
    arr.sort_by_key(|x| x.1);
    arr.reverse();
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    let mut ans = 0;
    for (val, min) in arr {
        sum += val as i64;
        heap.push(-val);
        if heap.len() > k {
            sum -= -heap.pop().unwrap() as i64;
        }
        if heap.len() == k {
            ans = ans.max(sum * min as i64);
        }
    }
    ans
}
