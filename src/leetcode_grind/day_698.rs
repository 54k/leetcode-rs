// https://leetcode.com/problems/maximal-score-after-applying-k-operations/description/?envType=daily-question&envId=2024-10-14
pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut pq = BinaryHeap::new();
    for num in nums {
        pq.push(num);
    }
    let mut ans = 0;
    while k > 0 {
        let top = pq.pop().unwrap();
        ans += top as i64;
        pq.push(((top as f64) / 3.0).ceil() as i32);
        k -= 1;
    }
    ans
}
