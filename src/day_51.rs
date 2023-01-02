#[allow(dead_code)]
pub fn detect_capital_use(word: String) -> bool {
    let n = word.len() as i32 - 1;
    let mut last_cap = -1;
    for (i, x) in word.chars().enumerate() {
        if !x.is_lowercase() {
            if i as i32 - last_cap > 1 {
                return false;
            } else {
                last_cap = i as i32;
            }
        }
    }
    last_cap <= 0 || last_cap == n
}

#[allow(dead_code)]
pub fn capitalize_title(title: String) -> String {
    let mut ans: Vec<String> = vec![];
    for w in title.split(' ') {
        let mut w = w.to_lowercase();
        if w.len() > 2 {
            w = w
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    if i == 0 {
                        ch.to_uppercase().last().unwrap()
                    } else {
                        ch
                    }
                })
                .collect();
        }
        ans.push(w);
    }
    ans.join(" ")
}

#[allow(dead_code)]
pub fn to_lower_case(s: String) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_uppercase() {
                ch.to_lowercase().last().unwrap()
            } else {
                ch
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut adj = vec![HashSet::new(); n as usize];

    for edge in edges {
        adj[edge[0] as usize].insert(edge[1] as usize);
        adj[edge[1] as usize].insert(edge[0] as usize);
    }

    let mut leaves = vec![];
    for (i, v) in adj.iter().enumerate() {
        if v.len() < 2 {
            leaves.push(i);
        }
    }

    while n > 2 {
        n -= leaves.len() as i32;
        let mut new_leaves = vec![];
        for leaf in leaves {
            let neighbor = *adj[leaf].iter().next().unwrap();
            adj[neighbor].remove(&leaf);
            if adj[neighbor].len() == 1 {
                new_leaves.push(neighbor);
            }
        }
        leaves = new_leaves;
    }
    leaves.into_iter().map(|i| i as _).collect()
}

#[allow(dead_code)]
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;
    let mut in_degree = vec![0; num_courses as usize];
    let mut adj = vec![vec![]; num_courses as usize];
    for prerequisite in prerequisites {
        adj[prerequisite[1] as usize].push(prerequisite[0] as usize);
        in_degree[prerequisite[0] as usize] += 1;
    }

    let mut q = VecDeque::new();
    for e in 0..num_courses as usize {
        if in_degree[e] == 0 {
            q.push_back(e);
        }
    }
    let mut size = 0;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        size += 1;
        for &u in &adj[v] {
            in_degree[u] -= 1;
            if in_degree[u] == 0 {
                q.push_back(u);
            }
        }
    }
    size == num_courses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test138() {
        println!("{}", detect_capital_use("USA".to_string()));
        println!("{}", detect_capital_use("FlaG".to_string()));
        println!("{}", detect_capital_use("Google".to_string()));
        println!("{}", detect_capital_use("okword".to_string()));
        println!(
            "{}",
            detect_capital_use("FFFFFFFFFFFFFFFFFFFFf".to_string())
        );
    }

    #[test]
    fn test139() {
        println!("{}", capitalize_title("capiTalIze tHe titLe".to_string()));
        println!(
            "{}",
            capitalize_title("First leTTeR of EACH Word".to_string())
        );
        println!("{}", capitalize_title("i lOve leetcode".to_string()));
    }

    #[test]
    fn test140() {
        println!(
            "{:?}",
            find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        ); // 1
        println!(
            "{:?}",
            find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            )
        ); // 3 4
        println!(
            "{:?}",
            find_min_height_trees(
                6,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![3, 4], vec![4, 5]]
            )
        ); // 3
        println!("{:?}", find_min_height_trees(1, vec![])); // 0
    }

    #[test]
    fn test141() {
        println!("{}", can_finish(2, vec![vec![1, 0]])); // true
        println!("{}", can_finish(1, vec![])); // true
    }
}
