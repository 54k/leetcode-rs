// https://leetcode.com/problems/take-gifts-from-the-richest-pile/description/?envType=daily-question&envId=2024-12-12
pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for &g in &gifts {
        heap.push(g);
    }
    while k > 0 {
        let mut x = heap.pop().unwrap();
        let sq = ((x as f64).sqrt()).floor() as i32;
        heap.push(sq);
        k -= 1;
    }
    heap.into_iter().map(|x| x as i64).sum::<i64>()
}
