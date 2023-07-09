// https://leetcode.com/problems/substring-with-largest-variance/description/
pub fn largest_variance(s: String) -> i32 {
    let mut counter = vec![0; 26];
    for ch in s.chars() {
        counter[ch as usize - 'a' as usize] += 1;
    }

    let mut global_max = 0;
    for i in 0..26 {
        for j in 0..26 {
            if i == j || counter[i] == 0 || counter[j] == 0 {
                continue;
            }

            let major = i;
            let minor = j;

            let mut major_count = 0;
            let mut minor_count = 0;

            let mut rest_minor = counter[j];

            for ch in s.chars() {
                let ch = ch as usize - 'a' as usize;

                if ch == major {
                    major_count += 1;
                } else if ch == minor {
                    minor_count += 1;
                    rest_minor -= 1;
                }

                if minor_count > 0 {
                    global_max = global_max.max(major_count - minor_count);
                }

                if major_count < minor_count && rest_minor > 0 {
                    major_count = 0;
                    minor_count = 0;
                }
            }
        }
    }

    global_max
}

// https://leetcode.com/problems/find-if-path-exists-in-graph/description/
pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut adj = vec![vec![]; n as usize];
    for edge in edges {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut stack = vec![source as usize];
    let mut seen = vec![false; n as usize];

    while let Some(v) = stack.pop() {
        if v == destination as usize {
            return true;
        }

        for &n in &adj[v] {
            if !seen[n] {
                seen[n] = true;
                stack.push(n);
            }
        }
    }

    false
}

// https://leetcode.com/problems/all-paths-from-source-to-target/description/
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn all_paths_source_target_backtrack(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn backtrack(
            source: usize,
            target: usize,
            results: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
            graph: &Vec<Vec<i32>>,
        ) {
            if source == target {
                results.push(path.clone());
                return;
            }

            for &next in &graph[source] {
                path.push(next);
                backtrack(next as usize, target, results, path, graph);
                path.pop();
            }
        }

        let target = graph.len() - 1;
        let mut results = vec![];
        let mut path = vec![0];

        backtrack(0, target, &mut results, &mut path, &graph);

        results
    }

    fn all_paths_source_target_dp(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        fn rec(
            memo: &mut HashMap<i32, Vec<Vec<i32>>>,
            curr_node: i32,
            target: i32,
            graph: &Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            if memo.contains_key(&curr_node) {
                return memo[&curr_node].clone();
            }

            let mut results = vec![];

            if curr_node == target {
                let mut path = vec![];
                path.push(target);
                results.push(path);
                return results;
            }

            for &next in &graph[curr_node as usize] {
                for path in rec(memo, next, target, graph) {
                    let mut new_path = vec![];
                    new_path.push(curr_node);
                    new_path.extend(path);
                    results.push(new_path);
                }
            }

            memo.insert(curr_node, results.clone());
            return results;
        }

        let mut memo = HashMap::new();
        let target = graph.len() - 1;
        rec(&mut memo, 0, target as i32, &graph)
    }
    all_paths_source_target_dp(graph)
}

// https://leetcode.com/problems/reconstruct-itinerary/description/
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    todo!("doesn't work");

    use std::collections::HashMap;

    let mut flight_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut visit_bitmap = HashMap::new();
    let mut result = vec![];

    // step 1 build the graph first
    for ticket in &tickets {
        let (origin, dest) = (ticket[0].clone(), ticket[1].clone());
        if flight_map.contains_key(&origin) {
            let mut dest_list = flight_map.get(&origin).unwrap().clone();
            dest_list.push(dest);
        } else {
            let mut dest_list = vec![];
            dest_list.push(dest);
            flight_map.insert(origin, dest_list);
        }
    }

    // step 2 order the destinations and init the visit bitmap
    for (k, v) in flight_map.iter_mut() {
        v.sort();
        visit_bitmap.insert(k.clone(), vec![false; v.len()]);
    }

    let flights = tickets.len();

    let mut route = vec![];
    route.push("JFK".to_string());

    // step 3 backtracking
    backtrack(
        "JFK".to_string(),
        &mut route,
        flights,
        &mut result,
        &flight_map,
        &mut visit_bitmap,
    );
    return result;

    fn backtrack(
        origin: String,
        route: &mut Vec<String>,
        flights: usize,
        result: &mut Vec<String>,
        flight_map: &HashMap<String, Vec<String>>,
        visit_bitmap: &mut HashMap<String, Vec<bool>>,
    ) -> bool {
        if route.len() == flights + 1 {
            *result = route.clone();
            return true;
        }

        if !flight_map.contains_key(&origin) {
            return false;
        }

        let mut i = 0usize;

        for dest in flight_map.get(&origin).unwrap() {
            if !visit_bitmap.get_mut(&origin).unwrap()[i] {
                visit_bitmap.get_mut(&origin).unwrap()[i] = true;
                route.push(dest.clone());
                let ret = backtrack(
                    dest.clone(),
                    route,
                    flights,
                    result,
                    flight_map,
                    &mut visit_bitmap.clone(),
                );
                route.pop();
                visit_bitmap.get_mut(&origin).unwrap()[i] = false;

                if ret {
                    return true;
                }
            }

            i += 1;
        }

        false
    }
}
