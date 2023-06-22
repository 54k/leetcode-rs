// https://leetcode.com/problems/minimum-cost-to-make-array-equal/description/
pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
    pub fn using_prefix_sum(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut nums_and_cost = vec![];
        for i in 0..n {
            nums_and_cost.push((nums[i], cost[i]));
        }
        nums_and_cost.sort_by(|a, b| a.0.cmp(&b.0));

        let mut prefix_cost = vec![0; n];
        prefix_cost[0] = nums_and_cost[0].1 as i64;
        for i in 1..n {
            prefix_cost[i] = prefix_cost[i - 1] as i64 + nums_and_cost[i].1 as i64;
        }

        let mut total_cost = 0;
        for i in 1..n {
            total_cost += (nums_and_cost[i].1 as i64
                * (nums_and_cost[i].0 as i64 - nums_and_cost[0].0 as i64))
                as i64;
        }
        let mut ans = total_cost;

        for i in 1..n {
            let gap = (nums_and_cost[i].0 - nums_and_cost[i - 1].0) as i64;
            total_cost += prefix_cost[i - 1] as i64 * gap;
            total_cost -= (prefix_cost[n - 1] as i64 - prefix_cost[i - 1] as i64) * gap;
            ans = ans.min(total_cost);
        }

        ans
    }
    pub fn using_convex_func(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        fn get_cost(nums: &Vec<i32>, cost: &Vec<i32>, base: i32) -> i64 {
            let mut ans = 0;
            for i in 0..nums.len() {
                ans += (nums[i] - base).abs() as i64 * cost[i] as i64;
            }
            ans
        }

        let (mut left, mut right) = (1000001, 0);
        for &num in &nums {
            left = left.min(num);
            right = right.max(num);
        }
        let mut ans = get_cost(&nums, &cost, nums[0]);

        while left < right {
            let mid = (right + left) / 2;
            let cost1 = get_cost(&nums, &cost, mid);
            let cost2 = get_cost(&nums, &cost, mid + 1);
            ans = cost1.min(cost2);

            if cost1 > cost2 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans
    }
    using_prefix_sum(nums, cost)
}