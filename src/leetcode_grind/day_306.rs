// https://leetcode.com/problems/reconstruct-itinerary/
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    use std::collections::HashMap;
    let mut adj = HashMap::new();
    for ticket in &tickets {
        let (from, to) = (ticket[0].clone(), ticket[1].clone());
        adj.entry(from).or_insert(vec![]).push(to);
    }
    let mut visited = HashMap::new();
    for (k, v) in &mut adj {
        v.sort();
        visited.entry(k.clone()).or_insert(vec![false; v.len()]);
    }

    let mut res = vec![];
    let mut path = vec!["JFK".to_string()];

    fn backtrack(
        from: String,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashMap<String, Vec<bool>>,
        path: &mut Vec<String>,
        res: &mut Vec<String>,
        tickets: usize,
    ) -> bool {
        if path.len() == tickets + 1 {
            *res = path.clone();
            return true;
        }
        if !adj.contains_key(&from) {
            return false;
        }
        for (i, to) in adj[&from].iter().enumerate() {
            if !visited[&from][i] {
                visited.get_mut(&from).unwrap()[i] = true;
                path.push(to.clone());
                let res = backtrack(to.clone(), adj, visited, path, res, tickets);
                path.pop();
                visited.get_mut(&from).unwrap()[i] = false;
                if res {
                    return true;
                }
            }
        }
        false
    }

    backtrack(
        "JFK".to_string(),
        &adj,
        &mut visited,
        &mut path,
        &mut res,
        tickets.len(),
    );

    res
}

pub fn find_itinerary_ii(tickets: Vec<Vec<String>>) -> Vec<String> {
    use std::collections::HashMap;
    let mut adj = HashMap::new();
    for ticket in tickets {
        let (from, to) = (ticket[0].clone(), ticket[1].clone());
        adj.entry(from).or_insert(vec![]).push(to);
    }

    fn dfs(
        from: String,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashMap<String, usize>,
        path: &mut Vec<String>,
    ) {
        if adj.contains_key(&from) {
            while visited[&from] < adj.get(&from).unwrap().len() {
                let to = adj[&from][visited[&from]].clone();
                *visited.get_mut(&from).unwrap() += 1;
                dfs(to, adj, visited, path);
            }
        }

        path.push(from);
    }

    let mut path = vec![];
    let mut visited = HashMap::new();
    for (k, v) in &mut adj {
        visited.insert(k.clone(), 0);
        v.sort();
    }

    dfs("JFK".to_owned(), &adj, &mut visited, &mut path);
    path.reverse();
    path
}
