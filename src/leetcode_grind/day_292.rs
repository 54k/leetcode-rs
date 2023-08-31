// https://leetcode.com/problems/parallel-courses-iii/description/
pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let mut in_degree = vec![0; n as usize];
    let mut adj = vec![vec![]; n as usize];
    for rel in relations {
        let from = (rel[0] - 1) as usize;
        let to = (rel[1] - 1) as usize;
        adj[from].push(to);
        in_degree[to] += 1;
    }

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut dist = vec![0; n as usize];

    for i in 0..n as usize {
        if in_degree[i] == 0 {
            queue.push_back(i);
            dist[i] = time[i];
        }
    }

    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            dist[v] = dist[v].max(dist[u] + time[v]);
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    dist.into_iter().max().unwrap()
}

#[test]
fn test_course_schedule() {
    let res = minimum_time(
        9,
        vec![
            vec![2, 7],
            vec![2, 6],
            vec![3, 6],
            vec![4, 6],
            vec![7, 6],
            vec![2, 1],
            vec![3, 1],
            vec![4, 1],
            vec![6, 1],
            vec![7, 1],
            vec![3, 8],
            vec![5, 8],
            vec![7, 8],
            vec![1, 9],
            vec![2, 9],
            vec![6, 9],
            vec![7, 9],
        ],
        vec![9, 5, 9, 5, 8, 7, 7, 8, 4],
    );
    println!("{res}"); // 32
}

// https://leetcode.com/problems/course-schedule-iii/description/
pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/jump-game-ii/description/
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = nums.len();

    let mut cur_end = 0;
    let mut cur_far = 0;

    for i in 0..n - 1 {
        cur_far = cur_far.max(i + nums[i] as usize);

        if i == cur_end {
            ans += 1;
            cur_end = cur_far;
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
pub fn min_taps_dp(n: i32, ranges: Vec<i32>) -> i32 {
    const INF: i32 = 1000_000_009;
    let mut dp = vec![i32::MAX; n as usize + 1];
    dp[0] = 0;

    for i in 0..ranges.len() {
        let tap_start = 0.max(i as i32 - ranges[i]) as usize;
        let tap_end = n.min(i as i32 + ranges[i]) as usize;

        for j in tap_start..tap_end {
            dp[tap_end] = dp[tap_end].min(dp[j] + 1);
        }
    }

    if dp[n as usize] == INF {
        return -1;
    }
    dp[n as usize]
}

pub fn min_taps_greedy(n: i32, ranges: Vec<i32>) -> i32 {
    let mut max_reach = vec![0; n as usize + 1];
    for i in 0..ranges.len() {
        let tap_start = 0.max(i as i32 - ranges[i]) as usize;
        let tap_end = n.min(i as i32 + ranges[i]) as usize;
        max_reach[tap_start] = max_reach[tap_start].max(tap_end);
    }

    let mut taps = 0;
    let mut curr_end = 0;
    let mut next_end = 0;

    for i in 0..=n as usize {
        if i > next_end {
            return -1;
        }

        if i > curr_end {
            taps += 1;
            curr_end = next_end;
        }

        next_end = next_end.max(max_reach[i]);
    }

    taps
}
