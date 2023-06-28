// https://leetcode.com/problems/path-with-maximum-probability/description/
pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
) -> f64 {
    pub fn max_probability_bellman_ford(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut max_prob = vec![0.0; n as usize];
        max_prob[start as usize] = 1.0;

        for _ in 0..n - 1 {
            let mut has_update = false;
            for j in 0..edges.len() {
                let (u, v) = (edges[j][0] as usize, edges[j][1] as usize);
                let path_prob = succ_prob[j];

                if max_prob[u] * path_prob > max_prob[v] {
                    max_prob[v] = max_prob[u] * path_prob;
                    has_update = true;
                }

                if max_prob[v] * path_prob > max_prob[u] {
                    max_prob[u] = max_prob[v] * path_prob;
                    has_update = true;
                }
            }

            if !has_update {
                break;
            }
        }

        max_prob[end as usize]
    }

    pub fn max_probability_spfa(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        use std::collections::VecDeque;

        let mut graph = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let (u, v, p) = (edges[i][0] as usize, edges[i][1] as usize, succ_prob[i]);
            graph[u].push((v, p));
            graph[v].push((u, p));
        }

        let mut max_prob = vec![0.0; n as usize];
        max_prob[start as usize] = 1.0;
        let mut queue = VecDeque::new();
        queue.push_back(start as usize);

        while let Some(node) = queue.pop_front() {
            for neighbor in &graph[node] {
                let (next, prob) = (neighbor.0, neighbor.1);

                if max_prob[node] * prob > max_prob[next] {
                    max_prob[next] = max_prob[node] * prob;
                    queue.push_back(next);
                }
            }
        }
        max_prob[end as usize]
    }

    max_probability_spfa(n, edges, succ_prob, start, end)
}

// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/description/
pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    const MOD: i64 = 1000_000_007;

    let mut graph = vec![vec![]; n as usize];

    for road in roads {
        let (u, v, cost) = (road[0] as usize, road[1] as usize, road[2]);
        graph[u].push((v, cost as i64));
        graph[v].push((u, cost as i64));
    }

    let mut heap = BinaryHeap::new();
    let mut dist = vec![(i64::MAX, 0); n as usize];
    dist[0] = (0, 1); // cost, ways
    heap.push(Reverse((0i64, 0usize))); // cost, node

    while let Some(Reverse(curr)) = heap.pop() {
        let (cost, v) = (curr.0, curr.1);

        for (nxt, nxt_cost) in &graph[v] {
            let nxt = *nxt;
            if cost + nxt_cost < dist[nxt].0 {
                dist[nxt] = (cost + nxt_cost, dist[v].1);
                heap.push(Reverse((cost + nxt_cost, nxt)));
            } else if cost + nxt_cost == dist[nxt].0 {
                dist[nxt].1 = (dist[nxt].1 % MOD + dist[v].1 % MOD) % MOD;
            }
        }
    }

    dist[n as usize - 1].1 as i32
}

// https://leetcode.com/problems/destroy-sequential-targets/description/
pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    use std::collections::HashMap;
    let mut max = i32::MIN;
    let mut hm = HashMap::new();
    for i in 0..nums.len() {
        let k = nums[i] % space;
        *hm.entry(k).or_insert(0) += 1;
        if hm[&k] > max {
            max = hm[&k];
        }
    }
    let mut ans = i32::MAX;
    for i in 0..nums.len() {
        let k = nums[i] % space;
        if hm[&k] == max && ans > nums[i] {
            ans = nums[i];
        }
    }
    ans
}

// https://leetcode.com/problems/burst-balloons/description/
pub fn max_coins(nums: Vec<i32>) -> i32 {
    pub fn max_coins_tle(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn dp(nums: Vec<i32>, cache: &mut HashMap<Vec<i32>, i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            }

            if cache.contains_key(&nums) {
                return cache[&nums];
            }

            let mut result = i32::MIN;
            for i in 0..nums.len() {
                let left = if i > 0 { nums[i - 1] } else { 1 };
                let right = if i < nums.len() - 1 { nums[i + 1] } else { 1 };
                let res = left * nums[i] * right;

                let mut new_state = nums.clone();
                new_state.remove(i);
                result = result.max(res + dp(new_state, cache));
            }
            cache.insert(nums.clone(), result);
            result
        }

        let mut cache = HashMap::new();
        dp(nums, &mut cache)
    }

    pub fn max_coins_top_down(nums: Vec<i32>) -> i32 {
        fn dp(memo: &mut Vec<Vec<i32>>, nums: &Vec<i32>, left: i32, right: i32) -> i32 {
            if right - left < 0 {
                return 0;
            }

            if memo[left as usize][right as usize] > 0 {
                return memo[left as usize][right as usize];
            }

            let mut result = 0;

            for i in left..=right {
                let gain = nums[left as usize - 1] * nums[i as usize] * nums[right as usize + 1];

                let remaining = dp(memo, nums, left, i as i32 - 1) + dp(memo, nums, i + 1, right);
                result = result.max(remaining + gain);
            }

            memo[left as usize][right as usize] = result;
            result
        }

        let n = nums.len() + 2;
        let mut new_nums = vec![1; n];
        for i in 0..nums.len() {
            new_nums[i + 1] = nums[i];
        }

        let mut memo = vec![vec![0; n]; n];

        dp(&mut memo, &new_nums, 1, n as i32 - 2)
    }

    pub fn max_coins_bottom_up(nums: Vec<i32>) -> i32 {
        // add 1 before and after nums
        let n = nums.len() + 2;
        let mut new_nums = vec![1; n];
        for i in 0..nums.len() {
            new_nums[i + 1] = nums[i];
        }
        // dp[i][j] represents
        // maximum if we burst all nums[left]...nums[right], inclusive
        let mut dp = vec![vec![0; n]; n];
        // do not include the first one and the last one
        // since they are both fake balloons added by ourselves and we can not burst
        // them
        for left in (1..n - 1).rev() {
            for right in left..n - 1 {
                // find the last burst one in newNums[left]...newNums[right]
                for i in left..=right {
                    // newNums[i] is the last burst one
                    let gain = new_nums[left - 1] * new_nums[i] * new_nums[right + 1];
                    // recursively call left side and right side
                    let remaining = dp[left][i - 1] + dp[i + 1][right];
                    // update
                    dp[left][right] = dp[left][right].max(remaining + gain);
                }
            }
        }
        dp[1][n - 2]
    }

    max_coins_top_down(nums)
}
