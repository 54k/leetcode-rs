// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/
pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
    pub fn max_value_top_down(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort();

        fn bisect(events: &Vec<Vec<i32>>, target: i32) -> usize {
            let mut left = 0;
            let mut right = events.len();
            while left < right {
                let mid = (left + right) / 2;
                if events[mid][0] <= target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        fn dfs(events: &Vec<Vec<i32>>, cur: usize, k: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if cur == events.len() || k == 0 {
                return 0;
            }

            if memo[cur][k] != -1 {
                return memo[cur][k];
            }

            memo[cur][k] = dfs(events, cur + 1, k, memo)
                .max(events[cur][2] + dfs(events, bisect(events, events[cur][1]), k - 1, memo));

            memo[cur][k]
        }

        let mut memo = vec![vec![-1; k as usize + 1]; events.len() as usize + 1];
        dfs(&events, 0, k as usize, &mut memo)
    }

    pub fn max_value_bottom_up(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort();

        fn bisect(events: &Vec<Vec<i32>>, target: i32) -> usize {
            let mut left = 0;
            let mut right = events.len();
            while left < right {
                let mid = (left + right) / 2;
                if events[mid][0] <= target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        let mut dp = vec![vec![0; k as usize + 1]; events.len() as usize + 1];

        for cur in (0..events.len()).rev() {
            let next_idx = bisect(&events, events[cur][1]);
            for count in 1..=k as usize {
                dp[cur][count] = dp[cur + 1][count].max(events[cur][2] + dp[next_idx][count - 1]);
            }
        }

        dp[0][k as usize]
    }

    max_value_bottom_up(events, k)
}
