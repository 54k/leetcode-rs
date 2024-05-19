// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description
pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut sum = 0i64;
    let mut count = 0;

    let mut positive_min = (1 << 30);
    let mut negative_max = -(1 << 30);

    for &node_val in &nums {
        let mut operated_node_value = node_val ^ k;
        sum += node_val as i64;
        let mut net_change = operated_node_value - node_val;

        if net_change > 0 {
            positive_min = positive_min.min(net_change);
            sum += net_change as i64;
            count += 1;
        } else {
            negative_max = negative_max.max(net_change);
        }
    }

    if count % 2 == 0 {
        return sum;
    }

    return (sum - positive_min as i64).max(sum + negative_max as i64);
}
