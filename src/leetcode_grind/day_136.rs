// https://leetcode.com/problems/minimum-cost-for-tickets/description/
// https://leetcode.com/problems/minimum-cost-for-tickets/editorial/
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    fn top_down_approach(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        fn rec(day: usize, day_set: &HashSet<usize>, memo: &mut [i32], costs: &[i32]) -> i32 {
            if day > 365 {
                return 0;
            }
            if memo[day] > -1 {
                return memo[day];
            }
            if day_set.contains(&day) {
                memo[day] = (rec(day + 1, day_set, memo, costs) + costs[0])
                    .min(rec(day + 7, day_set, memo, costs) + costs[1])
                    .min(rec(day + 30, day_set, memo, costs) + costs[2])
            } else {
                memo[day] = rec(day + 1, day_set, memo, costs);
            }
            memo[day]
        }
        use std::collections::HashSet;
        let day_set = days
            .iter()
            .copied()
            .map(|x| x as usize)
            .collect::<HashSet<_>>();
        let mut memo = vec![-1; 366];
        rec(1, &day_set, &mut memo, &costs)
    }
    fn bottom_up_approach(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut dp = vec![0; *days.last().unwrap() as usize + 1];
        let day_set = days
            .iter()
            .copied()
            .map(|x| x as usize)
            .collect::<HashSet<_>>();
        for i in 1..dp.len() {
            if day_set.contains(&i) {
                dp[i] = (dp[0.max(i as i32 - 1) as usize] + costs[0])
                    .min(dp[0.max(i as i32 - 7) as usize] + costs[1])
                    .min(dp[0.max(i as i32 - 30) as usize] + costs[2]);
            } else {
                dp[i] = dp[i - 1];
            }
        }
        *dp.last().unwrap()
    }
    top_down_approach(days, costs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test381() {
        println!(
            "{}",
            mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
        ); // 11
    }
}
