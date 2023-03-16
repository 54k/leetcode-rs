// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/solutions/75924/most-consistent-ways-of-dealing-with-the-series-of-stock-problems/?orderBy=most_relevant
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut t_ik0_pre = 0;
    let mut t_ik0 = 0;
    let mut t_ik1 = i32::MIN;

    for p in prices {
        let t_ik_old = t_ik0;
        t_ik0 = t_ik0.max(t_ik1 + p);
        t_ik1 = t_ik1.max(t_ik0_pre - p);
        t_ik0_pre = t_ik_old;
    }

    t_ik0
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test125() {
        println!("{:?}", max_profit(vec![1, 2, 3, 0, 2])); // 3
    }
}
