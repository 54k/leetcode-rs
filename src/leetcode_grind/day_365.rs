// https://leetcode.com/problems/bus-routes/description
pub fn num_buses_to_destination_1(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }

    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    let mut adj = HashMap::new();

    for r in 0..routes.len() as i32 {
        for &stop in &routes[r as usize] {
            adj.entry(stop).or_insert(vec![]).push(r);
        }
    }

    let mut queue = VecDeque::new();
    let mut vis = HashSet::new();

    for &route in &adj[&source] {
        queue.push_back(route);
        vis.insert(route);
    }

    let mut bus_count = 1;
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let route = queue.pop_front().unwrap() as usize;

            for &stop in &routes[route] {
                if stop == target {
                    return bus_count;
                }

                for &next_route in &adj[&stop] {
                    if !vis.contains(&next_route) {
                        vis.insert(next_route);
                        queue.push_back(next_route);
                    }
                }
            }
        }
        bus_count += 1;
    }

    -1
}

pub fn num_buses_to_destination_2(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }

    let mut adj = vec![];
    let mut routes = routes;

    for r in 0..routes.len() {
        routes[r].sort();
        adj.push(vec![]);
    }

    for i in 0..routes.len() {
        for j in i + 1..routes.len() {
            let r1 = &routes[i];
            let r2 = &routes[j];
            let mut r1i = 0;
            let mut r2i = 0;

            while r1i < r1.len() && r2i < r2.len() {
                if r1[r1i] == r2[r2i] {
                    adj[i].push(j);
                    adj[j].push(i);
                    break;
                }

                if r1[r1i] < r2[r2i] {
                    r1i += 1;
                } else {
                    r2i += 1;
                }
            }
        }
    }

    use std::collections::{HashSet, VecDeque};

    let mut vis = HashSet::new();
    let mut queue = VecDeque::new();

    for i in 0..routes.len() {
        for &r in &routes[i] {
            if r == source {
                queue.push_back(i);
                break;
            }
        }
    }

    let mut bus_count = 1;
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let node = queue.pop_front().unwrap();

            for &r in &routes[node] {
                if r == target {
                    return bus_count;
                }
            }

            for &next in &adj[node] {
                if !vis.contains(&next) {
                    vis.insert(next);
                    queue.push_back(next);
                }
            }
        }
        bus_count += 1;
    }

    -1
}

// https://leetcode.com/problems/sum-of-digits-in-the-minimum-number/description/
pub fn sum_of_digits_1(nums: Vec<i32>) -> i32 {
    (format!("{}", nums.into_iter().min().unwrap())
        .as_bytes()
        .into_iter()
        .map(|x| (x - b'0') as i32)
        .sum::<i32>()
        % 2
        - 1)
    .abs()
}

pub fn sum_of_digits_2(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    for n in nums {
        if n < min {
            min = n;
        }
    }
    let mut sum = 0;
    while min > 0 {
        sum += min % 10;
        min /= 10;
    }
    sum & 1 ^ 1
}
