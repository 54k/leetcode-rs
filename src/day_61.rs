pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    fn dfs_approach(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        fn dfs(
            v: i32,
            p: i32,
            adj: &Vec<Vec<i32>>,
            labels: &Vec<char>,
            ans: &mut Vec<i32>,
        ) -> Vec<i32> {
            let mut m1 = vec![0; 26];
            m1[labels[v as usize] as usize - 'a' as usize] = 1;
            if adj[v as usize].is_empty() {
                return m1;
            }
            for &u in &adj[v as usize] {
                if u != p {
                    let m2 = dfs(u, v, adj, labels, ans);
                    for (i, m) in m2.into_iter().enumerate() {
                        m1[i] += m;
                    }
                }
            }
            ans[v as usize] = m1[labels[v as usize] as usize - 'a' as usize];
            m1
        }

        let mut adj = vec![vec![]; n as usize];
        for e in edges {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        }
        let mut ans = vec![0; n as usize];
        dfs(0, -1, &adj, &labels.chars().collect(), &mut ans);
        ans
    }

    fn bfs_approach(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        use std::collections::VecDeque;

        let labels = labels.chars().collect::<Vec<_>>();

        let mut adj = vec![vec![]; n as usize];
        for e in edges {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        }

        let mut q = VecDeque::new();
        let mut ans = vec![0; n as usize];
        let mut counts = vec![vec![0; 26]; n as usize];

        for i in 0..n {
            counts[i as usize][labels[i as usize] as usize - 'a' as usize] = 1;
            if i > 0 && adj[i as usize].len() == 1 {
                q.push_back(i);
            }
        }

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            let p = *adj[v as usize].last().unwrap();
            adj[p as usize].retain(|f| *f != v);

            for i in 0..26 {
                counts[p as usize][i] += counts[v as usize][i];
            }

            if p > 0 && adj[p as usize].len() == 1 {
                q.push_back(p);
            }
        }

        for i in 0..n {
            ans[i as usize] = counts[i as usize][labels[i as usize] as usize - 'a' as usize];
        }

        ans
    }

    bfs_approach(n, edges, labels)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test155() {
        println!(
            "{:?}",
            count_sub_trees(
                5,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
                "aabab".to_string()
            )
        ); // [3,2,1,1,1]
    }
}
