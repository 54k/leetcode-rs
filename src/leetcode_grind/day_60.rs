pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    fn dfs(v: usize, p: usize, adj: &Vec<Vec<usize>>, has_apple: &Vec<bool>) -> i32 {
        let mut ans = 0;
        for &u in &adj[v] {
            if u != p {
                let x = dfs(u, v, adj, has_apple);
                if x > 0 || has_apple[u] {
                    ans += x + 2;
                }
            }
        }
        ans
    }
    let mut adj = vec![vec![]; n as usize];
    for e in edges {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }
    dfs(0, n as usize + 1, &adj, &has_apple)
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    use std::collections::HashSet;
    let mut ans = 0;
    let jewels = jewels.chars().collect::<HashSet<_>>();
    for s in stones.chars() {
        if jewels.contains(&s) {
            ans += 1;
        }
    }
    ans
}

pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let s = s.chars().collect::<Vec<_>>();
    let mut fc: i32 = -1;
    let mut bc: i32 = -1;
    let mut plate = 0;
    let mut f = vec![fc; s.len()];
    let mut b = vec![-1; s.len()];
    let mut c = vec![-1; s.len()];

    for (i, &ch) in s.iter().enumerate() {
        if ch == '*' {
            plate += 1;
        }
        c[i] = plate;
        if ch == '|' {
            fc = i as i32;
        }
        f[i] = fc;
    }
    for (i, &ch) in s.iter().enumerate().rev() {
        if ch == '|' {
            bc = i as i32;
        }
        b[i] = bc;
    }
    let mut ans = vec![];
    for q in queries {
        let p1 = b[q[0] as usize];
        let p2 = f[q[1] as usize];
        if p1 != -1 && p2 != -1 && p1 >= q[0] && p1 <= q[1] && p2 >= q[0] && p2 <= q[1] {
            ans.push(c[p2 as usize] - c[p1 as usize]);
        } else {
            ans.push(0);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test154() {
        println!(
            "{:?}",
            plates_between_candles(
                "***|**|*****|**||**|*".to_string(),
                vec![
                    vec![1, 17],
                    vec![4, 5],
                    vec![14, 17],
                    vec![5, 11],
                    vec![15, 16]
                ]
            )
        );
    }
}
