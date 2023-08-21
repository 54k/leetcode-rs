// https://leetcode.com/problems/cracking-the-safe/description/
pub fn crack_safe(n: i32, k: i32) -> String {
    use std::collections::HashSet;

    fn crack_safe_after(
        pwd: &mut String,
        visited_combi: &mut HashSet<String>,
        target_num_visited: usize,
        n: usize,
        k: usize,
    ) -> bool {
        if visited_combi.len() == target_num_visited {
            return true;
        }
        let last_digits = pwd[pwd.len() - n + 1..].to_string();
        // println!("last digits {}", last_digits);

        let to = char::from_u32('0' as u32 + k as u32).unwrap();
        // println!("to: {}", to);

        for ch in '0'..to {
            let new_comb = format!("{}{}", last_digits, ch);
            // println!("new comb: {}", new_comb);

            if !visited_combi.contains(&new_comb) {
                visited_combi.insert(new_comb.clone());
                pwd.push(ch);
                if crack_safe_after(pwd, visited_combi, target_num_visited, n, k) {
                    return true;
                }
                visited_combi.remove(&new_comb);
                pwd.pop();
            }
        }

        false
    }

    let mut str_pwd = "0".repeat(n as usize);
    let mut visited_combi = HashSet::new();
    visited_combi.insert(str_pwd.clone());
    let target_num_visited = k.pow(n as u32) as usize;

    crack_safe_after(
        &mut str_pwd,
        &mut visited_combi,
        target_num_visited,
        n as usize,
        k as usize,
    );

    str_pwd
}

// https://leetcode.com/problems/maximum-length-of-pair-chain/description/
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort();
    let mut dp = vec![0; pairs.len()];

    for i in 0..pairs.len() {
        dp[i] = dp[i].max(1);
        for j in 0..i {
            if pairs[j][1] < pairs[i][0] {
                dp[i] = dp[j] + 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..pairs.len() {
        ans = ans.max(dp[i]);
    }
    ans
}

#[test]
fn test_pairs_len() {
    let res = find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]);
    println!("{res}"); // 3
}

pub fn find_longest_chain_greedy(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by_key(|x| x[1]);
    let mut ans = 0;
    let mut curr_tail = i32::MIN;
    for i in 0..pairs.len() {
        if pairs[i][0] > curr_tail {
            curr_tail = pairs[i][1];
            ans += 1
        }
    }
    ans
}

// https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    fn find_prev_job(jobs: &Vec<i32>, last_start_time: i32) -> i32 {
        let mut start = 0;
        let mut end = jobs.len() as i32 - 1;
        let mut prev_idx = -1;

        while start <= end {
            let mid = (start + end) / 2;
            if jobs[mid as usize] > last_start_time {
                end = mid - 1;
            } else {
                prev_idx = mid;
                start = mid + 1;
            }
        }
        prev_idx
    }

    let mut jobs = end_time
        .into_iter()
        .zip(start_time)
        .zip(profit)
        .map(|((a, b), c)| (a, b, c))
        .collect::<Vec<_>>();
    jobs.sort();

    let end_times = jobs.iter().map(|x| x.0).collect::<Vec<_>>();

    let mut ans = 0;
    let mut dp = vec![0; jobs.len() + 1];
    for i in 1..=jobs.len() {
        let job_idx = i - 1;
        let prev_job = find_prev_job(&end_times, jobs[job_idx].1) + 1;
        let take_job = jobs[job_idx].2 + dp[prev_job as usize];
        dp[i] = take_job.max(dp[i - 1]);
        ans = ans.max(dp[i]);
    }

    ans
}

pub fn job_scheduling_heap(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut jobs = start_time
        .into_iter()
        .zip(end_time)
        .zip(profit)
        .map(|((s, e), p)| (s, e, p))
        .collect::<Vec<_>>();
    jobs.sort();

    let mut max_profit = 0;
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    for i in 0..jobs.len() {
        // println!("{:?}", heap);
        while !heap.is_empty() && heap.peek().unwrap().0 .0 <= jobs[i].0 {
            let top = heap.pop().unwrap().0;
            if top.1 > max_profit {
                max_profit = top.1;
            }
        }
        let combined_job = (jobs[i].1, jobs[i].2 + max_profit);
        heap.push(Reverse(combined_job));
    }

    while heap.len() > 0 {
        max_profit = max_profit.max(heap.pop().unwrap().0 .1);
    }

    max_profit
}
