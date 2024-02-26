// https://leetcode.com/problems/paths-in-maze-that-lead-to-same-room/description/
pub fn number_of_paths(n: i32, corridors: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let mut adj = HashMap::new();
    for i in 1..=n {
        adj.insert(i, HashSet::new());
    }
    for c in &corridors {
        adj.get_mut(&c[0]).unwrap().insert(c[1]);
        adj.get_mut(&c[1]).unwrap().insert(c[0]);
    }

    fn dfs(adj: &HashMap<i32, HashSet<i32>>, cur: i32, start: i32, cnt: i32) -> i32 {
        if cnt == 3 {
            if adj.get(&cur).unwrap().contains(&start) {
                return 1;
            }
            return 0;
        }

        let mut res = 0;
        for &u in adj.get(&cur).unwrap() {
            if u < cur {
                continue;
            }
            res += dfs(adj, u, start, cnt + 1);
        }
        res
    }

    let mut res = 0;
    for i in 1..=n {
        res += dfs(&adj, i, i, 1);
    }
    res
}
