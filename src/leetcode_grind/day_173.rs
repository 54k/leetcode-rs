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
        for (_, v) in map {
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
    map_approach(arr)
}

// https://leetcode.com/problems/grumpy-bookstore-owner/
// https://leetcode.com/problems/grumpy-bookstore-owner/solutions/299230/java-python-3-sliding-window/
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    // public int maxSatisfied(int[] customers, int[] grumpy, int X) {
    //     int satisfied = 0, maxMakeSatisfied = 0;
    //     for (int i = 0, winOfMakeSatisfied = 0; i < grumpy.length; ++i) {
    //         if (grumpy[i] == 0) { satisfied += customers[i]; }
    //         else { winOfMakeSatisfied += customers[i]; }
    //         if (i >= X) {
    //             winOfMakeSatisfied -= grumpy[i - X] * customers[i - X];
    //         }
    //         maxMakeSatisfied = Math.max(winOfMakeSatisfied, maxMakeSatisfied);
    //     }
    //     return satisfied + maxMakeSatisfied;
    // }

    let (mut satisfied, mut win_max_satisfied, mut max_satisfied) = (0, 0, 0);
    for i in 0..grumpy.len() {
        if grumpy[i] == 0 {
            satisfied += customers[i];
        } else {
            win_max_satisfied += customers[i];
        }
        if i >= minutes as usize {
            win_max_satisfied -= grumpy[i - minutes as usize] * customers[i - minutes as usize];
        }
        max_satisfied = max_satisfied.max(win_max_satisfied);
    }
    satisfied + max_satisfied
}

// https://leetcode.com/problems/decoded-string-at-index/
pub fn decode_at_index(s: String, k: i32) -> String {
    let mut size = 0;
    let mut k = k as i64;
    let s = s.chars().collect::<Vec<char>>();

    for &ch in &s {
        if char::is_digit(ch, 10) {
            size *= ch as i64 - '0' as i64;
        } else {
            size += 1;
        }
    }

    for ch in s.into_iter().rev() {
        k %= size;
        if k == 0 && char::is_ascii_alphabetic(&ch) {
            return format!("{}", ch);
        }

        if char::is_digit(ch, 10) {
            size /= ch as i64 - '0' as i64;
        } else {
            size -= 1;
        }
    }
    unreachable!();
}

// https://leetcode.com/problems/line-reflection/
pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;
    let mut m_points = HashSet::new();
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    for p in &points {
        let (x, y) = (p[0], p[1]);
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        m_points.insert((x, y));
    }
    let mid = min_x + max_x;

    for p in &points {
        let (x, y) = (p[0], p[1]);
        let delta_x = mid - x;
        if !m_points.contains(&(delta_x, y)) {
            return false;
        }
    }

    true
}

// https://leetcode.com/problems/total-cost-to-hire-k-workers/description/
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    fn using_2_heaps(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
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
            if tail.is_empty()
                || (!head.is_empty() && -head.peek().unwrap() <= -tail.peek().unwrap())
            {
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
    // meh, doesn't work, thanks to Rust's max-heaps
    fn using_1_heap(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..candidates as usize {
            heap.push((-costs[i], 0));
        }
        for i in (candidates as usize).max(costs.len() - candidates as usize)..costs.len() {
            heap.push((-costs[i], 1));
        }

        let mut cost = 0;
        let mut i = candidates as usize;
        let mut j = costs.len() - candidates as usize - 1;

        for _ in 0..k {
            let (cur_cost, cur_section_id) = heap.pop().unwrap();
            cost += -cur_cost as i64;
            if i <= j {
                if cur_section_id == 0 {
                    if i < costs.len() {
                        heap.push((-costs[i], 0));
                        i += 1;
                    }
                } else {
                    heap.push((-costs[j], 1));
                    j -= 1;
                }
            }
        }
        cost
    }
    using_2_heaps(costs, k, candidates)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test484() {
        println!("{}", restaurant_customers(vec![(5, 8), (2, 4), (3, 9)])); // 2
    }

    #[test]
    fn test485() {
        println!("{}", max_satisfied(vec![4, 10, 10], vec![1, 1, 0], 2)); // 24

        println!(
            "{}",
            max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            )
        ); // 16

        println!("{}", max_satisfied(vec![1], vec![0], 1)); // 16
    }

    #[test]
    fn test486() {
        println!("{}", decode_at_index("leet2code3".to_string(), 10)); // o
        println!("{}", decode_at_index("ha22".to_string(), 5)); // h
        println!(
            "{}",
            decode_at_index("a2345678999999999999999".to_string(), 1)
        ); // a
    }

    #[test]
    fn test488() {
        println!("{}", is_reflected(vec![vec![1, 1], vec![-1, -1]])); // false
        println!("{}", is_reflected(vec![vec![1, 1], vec![-1, 1]])); // true
        println!(
            "{}",
            is_reflected(vec![vec![-16, 1], vec![16, 1], vec![16, 1]])
        ); // true
    }

    #[test]
    fn test489() {
        println!("{}", total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)); // 11
        println!("{}", total_cost(vec![1, 2, 4, 1], 3, 3)); // 4
    }
}
