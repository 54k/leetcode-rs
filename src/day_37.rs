#[allow(dead_code)]
pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    fn dfs(
        e: usize,
        v: &mut Vec<bool>,
        adj: &Vec<Vec<usize>>,
        components: &mut Vec<i32>,
        cid: i32,
    ) {
        if v[e] {
            return;
        }
        v[e] = true;
        components[e] = cid;
        for c in &adj[e] {
            dfs(*c, v, adj, components, cid);
        }
    }

    let n = n as usize;
    let mut adj = vec![vec![]; n];

    for e in edges.iter() {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }

    let mut visited = vec![false; n];
    let mut components = vec![-1; n];

    for e in 0..n {
        if !visited[e] {
            dfs(e as usize, &mut visited, &adj, &mut components, e as i32);
        }
    }
    components[source as usize] == components[destination as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test121() {
        println!(
            "{}",
            valid_path(
                10,
                vec![
                    vec![4, 3],
                    vec![1, 4],
                    vec![4, 8],
                    vec![1, 7],
                    vec![6, 4],
                    vec![4, 2],
                    vec![7, 4],
                    vec![4, 0],
                    vec![0, 9],
                    vec![5, 4],
                ],
                5,
                9,
            )
        );
    }
}
