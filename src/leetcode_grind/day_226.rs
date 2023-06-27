// https://leetcode.com/problems/total-cost-to-hire-k-workers/description/
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut head = BinaryHeap::new();
    let mut tail = BinaryHeap::new();
    for i in 0..candidates as usize {
        head.push(-costs[i]);
    }
    for i in (candidates as usize).max(costs.len() - candidates as usize)..costs.len() {
        tail.push(-costs[i]);
    }
    let mut cost = 0;
    let mut i = candidates as usize;
    let mut j = costs.len() - candidates as usize - 1;

    for _ in 0..k {
        if tail.is_empty() || (!head.is_empty() && -head.peek().unwrap() <= -tail.peek().unwrap()) {
            cost += -head.pop().unwrap() as i64;
            if i < costs.len() && i <= j {
                head.push(-costs[i]);
                i += 1;
            }
        } else {
            cost += -tail.pop().unwrap() as i64;
            if i <= j {
                tail.push(-costs[j]);
                j -= 1;
            }
        }
    }
    cost
}
