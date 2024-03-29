pub fn sort_items(n: i32, m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut item_graph = HashMap::new();
    let mut item_in_degree = vec![0; n as usize];
    let mut group_graph = HashMap::new();

    let mut group_id = m;
    for i in 0..n as usize {
        item_graph.insert(i, vec![]);
        if group[i] == -1 {
            group[i] = group_id;
            group_id += 1;
        }
    }

    let mut group_in_degree = vec![0; group_id as usize];
    for i in 0..group_id as usize {
        group_graph.insert(i, vec![]);
    }

    for curr in 0..n as usize {
        for &prev in &before_items[curr] {
            item_graph.entry(prev as usize).or_insert(vec![]).push(curr);
            item_in_degree[curr] += 1;

            if group[curr] != group[prev as usize] {
                group_graph
                    .entry(group[prev as usize] as usize)
                    .or_insert(vec![])
                    .push(group[curr] as usize);
                group_in_degree[group[curr as usize] as usize] += 1;
            }
        }
    }

    fn topo_sort(graph: &HashMap<usize, Vec<usize>>, in_degree: &mut Vec<i32>) -> Vec<usize> {
        let mut stack = vec![];
        let mut visited = vec![];

        for &key in graph.keys() {
            if in_degree[key] == 0 {
                stack.push(key);
            }
        }

        while let Some(curr) = stack.pop() {
            visited.push(curr);
            for &prev in &graph[&curr] {
                in_degree[prev] -= 1;
                if in_degree[prev] == 0 {
                    stack.push(prev);
                }
            }
        }

        if visited.len() != graph.len() {
            return vec![];
        }

        visited
    }

    let item_order = topo_sort(&item_graph, &mut item_in_degree);
    let group_order = topo_sort(&group_graph, &mut group_in_degree);

    if item_order.is_empty() || group_order.is_empty() {
        return vec![];
    }

    let mut ordered_groups = HashMap::new();
    for item in item_order {
        ordered_groups
            .entry(group[item] as usize)
            .or_insert(vec![])
            .push(item);
    }
    let mut ans = vec![];
    for group_idx in group_order {
        ordered_groups.entry(group_idx).or_insert(vec![]);
        ans.extend(ordered_groups[&group_idx].clone());
    }
    ans.into_iter().map(|x| x as i32).collect()
}

// https://leetcode.com/problems/determine-the-minimum-sum-of-a-k-avoiding-array/
pub fn minimum_sum(n: i32, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut added = HashSet::new();
    let mut sum = 0;
    let mut i = 1;

    while (added.len() as i32) < n {
        if !added.contains(&(k - i)) {
            added.insert(i);
            sum += i;
        }
        i += 1;
    }

    sum
}

// https://leetcode.com/problems/find-the-longest-equal-subarray/
pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut count = HashMap::new();
    let mut max_freq = 0;
    let mut i = 0;

    for j in 0..nums.len() {
        *count.entry(nums[j]).or_insert(0) += 1;
        max_freq = max_freq.max(count[&nums[j]]);

        if j as i32 - i as i32 + 1 - max_freq > k {
            *count.entry(nums[i]).or_insert(0) -= 1;
            i += 1;
        }
    }
    max_freq
}

#[test]
fn test_longest_equal_subarray() {
    let res = longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3);
    println!("{res}"); // 3
}

// https://leetcode.com/problems/maximize-the-profit-as-the-salesman/description/
pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    let mut m = vec![];

    for _ in 0..n {
        m.push(vec![]);
    }

    for o in &offers {
        m[o[1] as usize].push(o.clone());
    }

    for e in 1..=n as usize {
        dp[e] = dp[e - 1];

        for a in &m[e - 1] {
            dp[e] = dp[e].max(dp[a[0] as usize] + a[2]);
        }
    }

    dp[n as usize]
}

// https://leetcode.com/problems/maximum-earnings-from-taxi/description/
pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    pub fn max_taxi_earnings_knapsack(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        rides.sort();
        let mut dp = vec![0; n as usize + 1];
        let mut j = 0;
        for i in 1..=n as usize {
            dp[i] = dp[i].max(dp[i - 1]);

            while j < rides.len() && rides[j][0] == i as i32 {
                let ride = &rides[j];
                dp[ride[1] as usize] = dp[ride[1] as usize]
                    .max(dp[i] + ride[1] as i64 - ride[0] as i64 + ride[2] as i64);
                j += 1;
            }
        }

        dp[n as usize]
    }
    pub fn max_taxi_earnings_sort_by_end(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides_by_end = vec![];
        for _ in 0..=n as usize {
            rides_by_end.push(vec![]);
        }
        for ride in &rides {
            rides_by_end[ride[1] as usize].push(ride.clone());
        }

        let mut dp = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            dp[i] = dp[i].max(dp[i - 1]);

            for ride in &rides_by_end[i] {
                dp[i] = dp[i]
                    .max(dp[ride[0] as usize] + ride[1] as i64 - ride[0] as i64 + ride[2] as i64);
            }
        }

        dp[n as usize]
    }
    max_taxi_earnings_sort_by_end(n, rides)
}

// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/description/
pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    events.sort();
    let mut j = 0;
    let mut res = 0;
    for d in 1..=100_000i32 {
        while !heap.is_empty() && heap.peek().unwrap().0 < d {
            heap.pop();
        }

        while j < events.len() && events[j][0] == d {
            heap.push(Reverse(events[j][1]));
            j += 1;
        }

        if !heap.is_empty() {
            heap.pop();
            res += 1;
        }
    }
    res
}

// https://leetcode.com/problems/meeting-rooms/description/
pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    intervals.sort();
    for i in 1..intervals.len() {
        if intervals[i - 1][1] > intervals[i][0] {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/find-right-interval/
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    for i in 0..intervals.len() {
        let mut min = i32::MAX;
        let mut min_index = -1;
        for j in 0..intervals.len() {
            if intervals[j][0] >= intervals[i][1] && intervals[j][0] < min {
                min_index = j as i32;
                min = intervals[j][0];
            }
        }
        res.push(min_index);
    }
    res
}

pub fn find_right_interval_ii(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut hash = HashMap::new();
    for i in 0..intervals.len() {
        hash.insert(intervals[i].clone(), i);
    }
    intervals.sort();
    let mut res = vec![0; intervals.len()];
    for i in 0..intervals.len() {
        let mut min = i32::MAX;
        let mut min_index = -1;
        for j in i..intervals.len() {
            if intervals[j][0] >= intervals[i][1] && intervals[j][0] < min {
                min_index = hash[&intervals[j]] as i32;
                min = intervals[j][0];
            }
        }
        res[hash[&intervals[i]]] = min_index as i32;
    }
    res
}

pub fn find_right_interval_iii(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::BTreeMap;
    let mut hash = BTreeMap::new();
    for (i, interval) in intervals.iter().enumerate() {
        hash.insert(interval[0], i);
    }

    let mut ans = vec![];

    for interval in &intervals {
        let ceil = hash
            .range(interval[1]..)
            .take(1)
            .map(|x| *x.1 as i32)
            .last()
            .unwrap_or(-1);
        ans.push(ceil);
    }

    ans
}

pub fn find_right_interval_iv(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut hash = HashMap::new();
    for (i, interval) in intervals.iter().enumerate() {
        hash.insert(interval.clone(), i);
    }
    let mut end_intervals = intervals.clone();
    intervals.sort();
    end_intervals.sort_by_key(|x| x[1]);
    let mut res = vec![-1; intervals.len()];
    let mut j = 0;
    for i in 0..end_intervals.len() {
        while j < intervals.len() && intervals[j][0] < end_intervals[i][1] {
            j += 1;
        }

        if j < intervals.len() {
            res[hash[&end_intervals[i]]] = hash[&intervals[j]] as i32;
        }
    }
    res
}

// https://leetcode.com/problems/is-array-a-preorder-of-some-binary-tree/
pub fn is_preorder(nodes: Vec<Vec<i32>>) -> bool {
    let mut stack = vec![vec![-1, -1]];
    for i in 0..nodes.len() {
        let child = nodes[i].clone();
        while stack.len() > 0 && stack.last().unwrap()[0] != child[1] {
            stack.pop();
            if stack.is_empty() {
                return false;
            }
        }
        stack.push(child);
    }
    true
}

#[test]
fn test_is_preorder() {
    let res = is_preorder(vec![
        vec![0, -1],
        vec![1, 0],
        vec![2, 0],
        vec![3, 2],
        vec![4, 2],
    ]);
    println!("{res}");
}

// https://leetcode.com/problems/create-target-array-in-the-given-order/
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut target = vec![];

    for i in 0..nums.len() {
        let idx = index[i] as usize;
        let num = nums[i];
        if target.len() == idx {
            target.push(num);
        } else {
            target.push(0);
            for j in (idx..target.len() - 1).rev() {
                target[j + 1] = target[j];
            }
            target[idx] = num;
        }
    }
    target
}

// https://leetcode.com/problems/contains-duplicate-iii/description/
pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
    use std::collections::BTreeSet;

    let mut set = BTreeSet::new();
    for i in 0..nums.len() {
        if let Some(&s) = set.range(nums[i]..).find(|_| true) {
            if s <= nums[i] + value_diff {
                return true;
            }
        }
        if let Some(&g) = set.range(..=nums[i]).last() {
            if nums[i] <= g + value_diff {
                return true;
            }
        }
        set.insert(nums[i]);
        if set.len() > index_diff as usize {
            set.remove(&nums[i - index_diff as usize]);
        }
    }
    false
}
