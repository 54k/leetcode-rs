// https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (0, arr.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if arr[mid] < arr[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i32
}

// https://leetcode.com/problems/parallel-courses/description/
pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let mut adj = vec![vec![]; n as usize + 1];
    let mut in_degree = vec![0; n as usize + 1];
    for r in relations {
        adj[r[0] as usize].push(r[1] as usize);
        in_degree[r[1] as usize] += 1;
    }

    let mut q = VecDeque::new();
    let mut next_q = VecDeque::new();
    let mut lvl = 0;

    for i in 1..=n as usize {
        if in_degree[i] == 0 {
            q.push_back(i);
        }
    }

    let mut topo = vec![];

    while q.len() > 0 {
        lvl += 1;
        while let Some(top) = q.pop_front() {
            topo.push(top);
            for &next in &adj[top] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    next_q.push_back(next);
                }
            }
        }
        q = next_q.clone();
        next_q.clear();
    }

    if topo.len() != n as usize {
        return -1;
    }

    return lvl;
}
