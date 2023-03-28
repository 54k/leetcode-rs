// https://leetcode.com/problems/minimum-cost-for-tickets/description/
// https://leetcode.com/problems/minimum-cost-for-tickets/editorial/
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
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
