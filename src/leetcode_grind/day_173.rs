// https://leetcode.com/problems/richest-customer-wealth/
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .into_iter()
        .map(|v| v.into_iter().sum::<i32>())
        .max()
        .unwrap()
}

// https://beingcodeexpert.blogspot.com/2021/05/restaurant-customers.html
// Input:  n = 3, arr = {{5, 8}, {2, 4}, {3, 9}}
// Output: 2
pub fn restaurant_customers(arr: Vec<(i32, i32)>) -> i32 {
    fn map_approach(arr: Vec<(i32, i32)>) -> i32 {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for (l, r) in arr {
            *map.entry(l).or_insert(0) += 1;
            *map.entry(r + 1).or_insert(0) -= 1;
        }
        let mut cur = 0;
        let mut ans = 0;
        for (k, v) in map {
            cur += v;
            ans = ans.max(cur);
        }
        ans
    }
    fn arr_approach(mut arr: Vec<(i32, i32)>) -> i32 {
        arr.sort();
        let mut ans = arr[0].0;
        let mut max = 1;
        let mut num_of_ppl = 0;
        let mut i = 1;
        let mut j = 0;
        while i < arr.len() {
            if arr[i].0 <= arr[j].1 {
                i += 1;
                num_of_ppl += 1;
                if num_of_ppl > max {
                    max = num_of_ppl;
                    ans = arr[i].0;
                }
            } else {
                j += 1;
                num_of_ppl -= 1;
            }
        }
        ans
    }
    arr_approach(arr)
}

// https://leetcode.com/problems/grumpy-bookstore-owner/
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    todo!();
}

// https://leetcode.com/problems/decoded-string-at-index/
pub fn decode_at_index(s: String, k: i32) -> String {
    todo!();
}

// https://leetcode.com/problems/line-reflection/
pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
    todo!();
}

// https://leetcode.com/problems/total-cost-to-hire-k-workers/description/
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test484() {
        println!("{}", restaurant_customers(vec![(5, 8), (2, 4), (3, 9)])); // 2
    }
}
