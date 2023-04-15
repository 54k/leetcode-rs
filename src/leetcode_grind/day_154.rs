// https://leetcode.com/problems/open-the-lock/
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    fn get_neighbors(s: String) -> Vec<String> {
        let mut res = vec![];
        for (id, i) in s.chars().enumerate() {
            for j in [1, -1] {
                let mut ch = i as i32 - '0' as i32;
                ch = (ch + j + 10) % 10;
                res.push(format!("{}{}{}", &s[0..id], ch, &s[id + 1..]));
            }
        }
        res
    }
    use std::collections::{HashSet, VecDeque};
    let mut seen = HashSet::new();
    for d in deadends {
        if d == *"0000" {
            return -1;
        }
        seen.insert(d);
    }
    let mut queue = VecDeque::new();
    queue.push_back(("0000".to_string(), 0));
    seen.insert("0000".to_string());
    while let Some((v, steps)) = queue.pop_front() {
        if v == target {
            return steps;
        }
        for u in get_neighbors(v) {
            if !seen.contains(&u) {
                seen.insert(u.clone());
                queue.push_back((u, steps + 1));
            }
        }
    }
    -1
}

// https://leetcode.com/problems/evaluate-division/
pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    fn search(start: String, end: String, graph: &HashMap<String, HashMap<String, f64>>) -> f64 {
        if !graph.contains_key(&start) {
            return -1.;
        }
        use std::collections::HashSet;
        let mut stack = vec![];
        let mut seen = HashSet::new();
        stack.push((&start, 1.));
        seen.insert(&start);
        while let Some((v, ratio)) = stack.pop() {
            if v == &end {
                return ratio;
            }
            for (u, r) in &graph[v] {
                if !seen.contains(u) {
                    seen.insert(u);
                    stack.push((u, ratio * r))
                }
            }
        }
        -1.
    }
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    for (i, equ) in equations.into_iter().enumerate() {
        let num = equ[0].clone();
        let div = equ[1].clone();
        graph
            .entry(num.clone())
            .or_insert(HashMap::new())
            .insert(div.clone(), values[i]);
        graph
            .entry(div.clone())
            .or_insert(HashMap::new())
            .insert(num.clone(), 1. / values[i]);
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        ans.push(search(q[0].clone(), q[1].clone(), &graph))
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test439() {
        println!(
            "{}",
            open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            )
        ); // 6

        println!(
            "{}",
            open_lock(vec!["8888".to_string()], "0009".to_string())
        ); // 1

        println!(
            "{}",
            open_lock(
                vec![
                    "8887".to_string(),
                    "8889".to_string(),
                    "8878".to_string(),
                    "8898".to_string(),
                    "8788".to_string(),
                    "8988".to_string(),
                    "7888".to_string(),
                    "9888".to_string()
                ],
                "8888".to_string()
            )
        ); // -1
    }

    #[test]
    fn test440() {
        println!(
            "{:?}",
            calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()],
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "e".to_string()],
                    vec!["a".to_string(), "a".to_string()],
                    vec!["x".to_string(), "x".to_string()],
                ],
            )
        ); // [6.00000,0.50000,-1.00000,1.00000,-1.00000]
    }
}
