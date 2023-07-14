// https://leetcode.com/problems/network-delay-time/description/
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    pub fn network_delay_time_dfs(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        fn dfs(
            adj: &Vec<Vec<(usize, i32)>>,
            signal_received_at: &mut Vec<i32>,
            cur_node: usize,
            cur_time: i32,
        ) {
            if cur_time >= signal_received_at[cur_node] {
                return;
            }

            signal_received_at[cur_node] = cur_time;

            for (neighbor, travel_time) in &adj[cur_node] {
                dfs(adj, signal_received_at, *neighbor, cur_time + *travel_time);
            }
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for time in &times {
            let (f, t, w) = (time[0] as usize, time[1] as usize, time[2]);
            adj[f].push((t, w));
        }
        for edges in &mut adj {
            edges.sort_by_key(|x| x.1);
        }

        let mut signal_received_at = vec![i32::MAX; n as usize + 1];

        dfs(&adj, &mut signal_received_at, k as usize, 0);

        let mut ans = i32::MIN;
        for node in 1..=n as usize {
            ans = signal_received_at[node].max(ans);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }

    pub fn network_delay_time_bfs(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        fn bfs(
            adj: &Vec<Vec<(usize, i32)>>,
            signal_received_at: &mut Vec<i32>,
            source_node: usize,
        ) {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(source_node);
            signal_received_at[source_node] = 0;

            while let Some(curr_node) = queue.pop_front() {
                for (neighbor, travel_time) in &adj[curr_node] {
                    let arrival_time = signal_received_at[curr_node] + *travel_time;
                    if signal_received_at[*neighbor] > arrival_time {
                        signal_received_at[*neighbor] = arrival_time;
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for time in &times {
            let (f, t, w) = (time[0] as usize, time[1] as usize, time[2]);
            adj[f].push((t, w));
        }
        for edges in &mut adj {
            edges.sort_by_key(|x| x.1);
        }

        let mut signal_received_at = vec![i32::MAX; n as usize + 1];

        bfs(&adj, &mut signal_received_at, k as usize);

        let mut ans = i32::MIN;
        for node in 1..=n as usize {
            ans = signal_received_at[node].max(ans);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }

    pub fn network_delay_time_dijkstra(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        fn dijkstra(
            adj: &Vec<Vec<(usize, i32)>>,
            signal_received_at: &mut Vec<i32>,
            source_node: usize,
        ) {
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;

            let mut pq = BinaryHeap::new();
            signal_received_at[source_node] = 0;
            pq.push(Reverse((0, source_node)));

            while let Some(Reverse((curr_node_time, curr_node))) = pq.pop() {
                if curr_node_time > signal_received_at[curr_node] {
                    continue;
                }

                for (neighbor, travel_time) in &adj[curr_node] {
                    if signal_received_at[*neighbor] > curr_node_time + *travel_time {
                        signal_received_at[*neighbor] = curr_node_time + *travel_time;
                        pq.push(Reverse((signal_received_at[*neighbor], *neighbor)));
                    }
                }
            }
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for time in &times {
            let (f, t, w) = (time[0] as usize, time[1] as usize, time[2]);
            adj[f].push((t, w));
        }

        let mut signal_received_at = vec![i32::MAX; n as usize + 1];

        dijkstra(&adj, &mut signal_received_at, k as usize);

        let mut ans = i32::MIN;
        for node in 1..=n as usize {
            ans = signal_received_at[node].max(ans);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
    network_delay_time_dijkstra(times, n, k)
}

// https://leetcode.com/problems/the-time-when-the-network-becomes-idle/description/
pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let n = patience.len();
    let mut adj = vec![vec![]; n];

    for e in &edges {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }

    let mut dist_to_master = vec![-1; n];
    dist_to_master[0] = 0;

    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(curr) = q.pop_front() {
        for &next in &adj[curr] {
            if dist_to_master[next] == -1 {
                dist_to_master[next] = dist_to_master[curr] + 1;
                q.push_back(next);
            }
        }
    }

    let mut idle = 0;

    for i in 1..n {
        let first_message_recv = dist_to_master[i] * 2;
        let num_of_extra = (dist_to_master[i] * 2 - 1) / patience[i];
        let last_msg_sent = num_of_extra * patience[i];
        idle = idle.max(last_msg_sent + first_message_recv);
    }

    idle + 1
}
