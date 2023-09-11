// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/description
pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    let mut ans = vec![];
    let mut groups = HashMap::new();
    for (i, &gs) in group_sizes.iter().enumerate() {
        groups.entry(gs).or_insert(vec![]).push(i as i32);
    }
    for (k, mut v) in groups {
        while v.len() > 0 {
            let mut cur = vec![];
            while cur.len() as i32 != k {
                cur.push(v.pop().unwrap());
            }
            ans.push(cur);
        }
    }
    ans
}

// https://leetcode.com/problems/sell-diminishing-valued-colored-balls/description/
pub fn max_profit_i_tle(inventory: Vec<i32>, orders: i32) -> i32 {
    use std::collections::BinaryHeap;
    const MOD: i64 = 1000000007;
    let mut orders = orders;
    let mut heap = BinaryHeap::new();
    for i in inventory {
        heap.push(i);
    }

    let mut sum = 0;

    while orders > 0 {
        let top = heap.pop().unwrap();
        sum += top as i64;
        sum %= MOD;
        heap.push(top - 1);
        orders -= 1;
    }

    sum as i32
}

// https://leetcode.com/problems/sell-diminishing-valued-colored-balls/solutions/927716/c-o-n-log-n/
pub fn max_profit_ii(inventory: Vec<i32>, orders: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let mut inventory = inventory;
    inventory.sort();

    let mut orders = orders;

    let mut res = 0;
    let mut colors = 1;

    for i in (0..inventory.len()).rev() {
        if orders <= 0 {
            break;
        }

        let cur = inventory[i];
        let prev = if i > 0 { inventory[i - 1] } else { 0 };

        let rounds = (orders / colors).min(cur - prev);
        orders -= rounds * colors;

        let sum =
            (cur as i64 * (cur + 1) as i64 - (cur - rounds) as i64 * (cur - rounds + 1) as i64) / 2
                * colors as i64;
        res += sum;
        res %= MOD;

        if cur - prev > rounds {
            let sum = orders as i64 * (cur - rounds) as i64;
            res += sum;
            res %= MOD;
            break;
        }

        colors += 1;
    }

    res as i32
}

#[test]
fn test_max_profit() {
    let res = max_profit_ii(vec![2, 5], 4);
    println!("{res}");
}

// https://leetcode.com/problems/split-array-largest-sum/description/
pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut memo = vec![vec![0; 51]; 1001];
    let n = nums.len();
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] += prefix_sum[i] + nums[i];
    }

    for subarray_count in 1..=k as usize {
        for curr_index in 0..n {
            if subarray_count == 1 {
                memo[curr_index][subarray_count] = prefix_sum[n] - prefix_sum[curr_index];
                continue;
            }

            let mut minimum_largest_split_sum = i32::MAX;
            for i in curr_index..=n - subarray_count {
                let first_split_sum = prefix_sum[i + 1] - prefix_sum[curr_index];

                let largest_split_sum = first_split_sum.max(memo[i + 1][subarray_count - 1]);

                minimum_largest_split_sum = minimum_largest_split_sum.min(largest_split_sum);

                if first_split_sum >= minimum_largest_split_sum {
                    break;
                }
            }

            memo[curr_index][subarray_count] = minimum_largest_split_sum;
        }
    }

    memo[0][k as usize]
}
