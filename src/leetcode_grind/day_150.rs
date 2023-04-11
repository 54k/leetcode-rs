// https://leetcode.com/problems/maximum-number-of-robots-within-budget/
pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
    use std::collections::VecDeque;
    let mut ans = 0;
    let mut p = 0;
    let mut stack = VecDeque::new();
    let mut start = 0;
    for end in 0..charge_times.len() {
        p += running_costs[end] as i64;
        while !stack.is_empty() && *stack.back().unwrap() < charge_times[end] {
            let _ = stack.pop_back().unwrap();
        }
        stack.push_back(charge_times[end]);

        while !stack.is_empty()
            && (*stack.front().unwrap() as i64 + (end as i32 - start as i32 + 1) as i64 * p)
                > budget
        {
            if charge_times[start] == *stack.front().unwrap() {
                stack.pop_front().unwrap();
            }
            p -= running_costs[start] as i64;
            start += 1;
        }

        let k = end as i32 - start as i32 + 1;
        ans = ans.max(k);
    }
    ans
}

// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    fn backwards(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![];
        for i in (0..prices.len()).rev() {
            while !stack.is_empty() && *stack.last().unwrap() > prices[i] {
                stack.pop().unwrap();
            }
            ans.push(prices[i] - *stack.last().unwrap_or(&0));
            stack.push(prices[i]);
        }
        ans.reverse();
        ans
    }
    fn forward(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        for i in 0..prices.len() {
            while !stack.is_empty() && prices[*stack.last().unwrap()] >= prices[i] {
                prices[stack.pop().unwrap()] -= prices[i];
            }
            stack.push(i);
        }
        prices
    }
    forward(prices)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test430() {
        println!(
            "{}",
            maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25)
        ); // 3
        println!("{}", maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19)); // 0
        println!(
            "{}",
            maximum_robots(
                vec![
                    19, 63, 21, 8, 5, 46, 56, 45, 54, 30, 92, 63, 31, 71, 87, 94, 67, 8, 19, 89,
                    79, 25
                ],
                vec![
                    91, 92, 39, 89, 62, 81, 33, 99, 28, 99, 86, 19, 5, 6, 19, 94, 65, 86, 17, 10,
                    8, 42
                ],
                85
            )
        ); // 1
        println!(
            "{}",
            maximum_robots(
                vec![11, 12, 74, 67, 37, 87, 42, 34, 18, 90, 36, 28, 34, 20],
                vec![18, 98, 2, 84, 7, 57, 54, 65, 59, 91, 7, 23, 94, 20],
                937
            )
        ); // 4
    }

    #[test]
    fn test431() {
        println!("{:?}", final_prices(vec![8, 4, 6, 2, 3])); // [4,2,4,2,3]
    }
}
