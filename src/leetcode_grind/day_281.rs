// https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/description/
pub fn sort_items(n: i32, m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut item_graph = HashMap::new();
    let mut item_in_degree = vec![0; n as usize];
    let mut group_graph = HashMap::new();

    let mut group_id = m;
    for i in 0..n as usize {
        item_graph.insert(i, vec![]);
        if group[i] == -1 {
            group[i] = group_id;
            group_id += 1;
        }
    }

    let mut group_in_degree = vec![0; group_id as usize];
    for i in 0..group_id as usize {
        group_graph.insert(i, vec![]);
    }

    for curr in 0..n as usize {
        for &prev in &before_items[curr] {
            item_graph.entry(prev as usize).or_insert(vec![]).push(curr);
            item_in_degree[curr] += 1;

            if group[curr] != group[prev as usize] {
                group_graph
                    .entry(group[prev as usize] as usize)
                    .or_insert(vec![])
                    .push(group[curr] as usize);
                group_in_degree[group[curr as usize] as usize] += 1;
            }
        }
    }

    fn topo_sort(graph: &HashMap<usize, Vec<usize>>, in_degree: &mut Vec<i32>) -> Vec<usize> {
        let mut stack = vec![];
        let mut visited = vec![];

        for &key in graph.keys() {
            if in_degree[key] == 0 {
                stack.push(key);
            }
        }

        while let Some(curr) = stack.pop() {
            visited.push(curr);
            for &prev in &graph[&curr] {
                in_degree[prev] -= 1;
                if in_degree[prev] == 0 {
                    stack.push(prev);
                }
            }
        }

        if visited.len() != graph.len() {
            return vec![];
        }

        visited
    }

    let item_order = topo_sort(&item_graph, &mut item_in_degree);
    let group_order = topo_sort(&group_graph, &mut group_in_degree);

    if item_order.is_empty() || group_order.is_empty() {
        return vec![];
    }

    let mut ordered_groups = HashMap::new();
    for item in item_order {
        ordered_groups
            .entry(group[item] as usize)
            .or_insert(vec![])
            .push(item);
    }
    let mut ans = vec![];
    for group_idx in group_order {
        ordered_groups.entry(group_idx).or_insert(vec![]);
        ans.extend(ordered_groups[&group_idx].clone());
    }
    ans.into_iter().map(|x| x as i32).collect()
}

pub fn minimum_sum(n: i32, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut added = HashSet::new();
    let mut sum = 0;
    let mut i = 1;

    while (added.len() as i32) < n {
        if !added.contains(&(k - i)) {
            added.insert(i);
            sum += i;
        }
        i += 1;
    }

    sum
}

pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut count = HashMap::new();
    let mut max_freq = 0;
    let mut i = 0;

    for j in 0..nums.len() {
        *count.entry(nums[j]).or_insert(0) += 1;
        max_freq = max_freq.max(count[&nums[j]]);

        if j as i32 - i as i32 + 1 - max_freq > k {
            *count.entry(nums[i]).or_insert(0) -= 1;
            i += 1;
        }
    }
    max_freq
}

#[test]
fn test_longest_equal_subarray() {
    let res = longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3);
    println!("{res}"); // 3
}
