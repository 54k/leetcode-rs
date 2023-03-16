pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    fn find(cmp: &mut Vec<usize>, x: usize) -> usize {
        if x != cmp[x] {
            cmp[x] = find(cmp, cmp[x]);
        }
        cmp[x]
    }
    fn union(cmp: &mut Vec<usize>, x: usize, y: usize) {
        let mut x = find(cmp, x);
        let mut y = find(cmp, y);
        if x < y {
            std::mem::swap(&mut x, &mut y);
        }
        cmp[x] = y;
    }

    use std::collections::BTreeMap;

    let n = vals.len();
    let mut good_paths = 0;
    let mut cmp = (0..n).collect::<Vec<_>>();
    let mut val_to_nodes = BTreeMap::new();
    let mut adj = vec![vec![]; n];

    for e in edges {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }

    for i in 0..n {
        val_to_nodes.entry(vals[i]).or_insert(vec![]).push(i);
    }

    for val in val_to_nodes.keys() {
        val_to_nodes.get(val).iter().for_each(|&v| {
            for &node in v {
                for &u in &adj[node] {
                    if vals[node] >= vals[u] {
                        union(&mut cmp, node, u);
                    }
                }
            }
        });

        let mut groups = vec![0; n];
        for &i in val_to_nodes.get(val).unwrap() {
            groups[find(&mut cmp, i)] += 1;
        }

        groups.into_iter().for_each(|v| {
            let sum = (v * (v + 1)) / 2;
            good_paths += sum
        })
    }

    good_paths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test160() {
        println!(
            "{}",
            number_of_good_paths(
                vec![1, 3, 2, 1, 3],
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
            )
        ); // 6
        println!(
            "{}",
            number_of_good_paths(
                vec![2, 4, 3, 1, 5],
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4]]
            )
        ); // 5
    }
}
