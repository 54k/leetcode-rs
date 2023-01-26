// https://leetcode.com/problems/cheapest-flights-within-k-stops/solutions/2825208/cheapest-flights-within-k-stops/
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    fn bellman_ford(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut dist = vec![i32::MAX; n as usize];
        dist[src as usize] = 0;

        for _ in 0..=k {
            let mut tmp = dist.clone();
            for flight in flights.iter() {
                let from_dist = dist[flight[0] as usize];
                let to_dist = tmp[flight[1] as usize];
                if from_dist < i32::MAX && to_dist >= from_dist + flight[2] {
                    tmp[flight[1] as usize] = from_dist + flight[2];
                }
            }
            dist = tmp;
        }

        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }

    fn bfs(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::*;
        let mut adj = vec![vec![]; n as usize];
        for f in flights {
            adj[f[0] as usize].push((f[1] as usize, f[2]));
        }

        let mut stops = 0;
        let mut dist = vec![i32::MAX; n as usize];
        let mut q = VecDeque::new();
        q.push_back((src as usize, 0));

        while stops <= k && !q.is_empty() {
            let mut s = q.len();
            while s > 0 {
                let v = q.pop_front().unwrap();
                for &u in &adj[v.0] {
                    if dist[u.0] >= v.1 + u.1 {
                        dist[u.0] = v.1 + u.1;
                        q.push_back((u.0, dist[u.0]));
                    }
                }
                s -= 1;
            }
            stops += 1;
        }

        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }

    fn dijkstra(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::*;
        let mut adj = vec![vec![]; n as usize];
        for f in flights {
            adj[f[0] as usize].push((f[1] as usize, f[2]));
        }

        let mut dist = vec![i32::MAX; n as usize];
        let mut q = BinaryHeap::new();
        q.push((0, src as usize, 0));

        while let Some(v) = q.pop() {
            if v.2 > k {
                continue;
            }
            for &u in &adj[v.1] {
                if dist[u.0] >= v.0 + u.1 {
                    dist[u.0] = v.0 + u.1;
                    q.push((dist[u.0], u.0, v.2 + 1));
                }
            }
        }

        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }

    bellman_ford(n, flights, src, dst, k)
    // bfs(n, flights, src, dst, k)
    // dijkstra(n, flights, src, dst, k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test177() {
        println!(
            "{}",
            find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        ); // 700

        println!(
            "{}",
            find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            )
        ); // 200

        println!(
            "{}",
            find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            )
        ); // 500
    }
}
