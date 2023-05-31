// https://leetcode.com/problems/min-cost-climbing-stairs/description/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (mut a, mut b) = (cost[0], cost[1]);
    for i in 2..=cost.len() {
        let c = if i == cost.len() { 0 } else { cost[i] } + a.min(b);
        a = b;
        b = c;
    }
    b
}
