// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/description/
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len();
    let mut count = 0;
    let mut row_neg_idx = n as i32 - 1;

    for row in grid {
        while row_neg_idx >= 0 && row[row_neg_idx as usize] < 0 {
            row_neg_idx -= 1;
        }
        count += (n as i32 - row_neg_idx - 1) as i32;
    }

    count
}

// https://leetcode.com/discuss/interview-question/algorithms/125398/given-a-stack-sort-it-in-non-decreasing-order
pub fn sort_stack(mut s: Vec<i32>) -> Vec<i32> {
    let mut t = vec![];
    while let Some(top) = s.pop() {
        while !t.is_empty() && *t.last().unwrap() < top {
            s.push(t.pop().unwrap());
        }
        t.push(top);
    }
    while let Some(top) = t.pop() {
        s.push(top);
    }
    s
}

// https://leetcode.com/problems/minimum-cost-to-reach-city-with-discounts/description/
pub fn minimum_cost(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut adj = vec![vec![-1; n as usize]; n as usize];
    for e in &highways {
        adj[e[0] as usize][e[1] as usize] = e[2];
        adj[e[1] as usize][e[0] as usize] = e[2];
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, discounts as usize)));

    let mut ans = vec![i32::MAX; n as usize];
    let mut visited = vec![vec![false; n as usize]; discounts as usize + 1];

    while let Some(Reverse((d1, from, remain))) = heap.pop() {
        if visited[remain][from] {
            continue;
        }
        visited[remain][from] = true;
        ans[from] = ans[from].min(d1);
        if from == n as usize - 1 {
            break;
        }
        for (to, &w) in adj[from].iter().enumerate() {
            if w == -1 {
                continue;
            }
            if remain > 0 {
                heap.push(Reverse((d1 + w / 2, to, remain - 1)));
            }
            heap.push(Reverse((d1 + w, to, remain)));
        }
    }
    if ans[n as usize - 1] == i32::MAX {
        -1
    } else {
        ans[n as usize - 1]
    }
}

// https://www.geeksforgeeks.org/minimum-cost-path-in-a-directed-graph-via-given-set-of-intermediate-nodes/
pub fn minimum_cost_with_points_of_interest(
    n: i32,
    edges: Vec<Vec<i32>>,
    points: Vec<i32>,
) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let p_len: i32 = points.len() as i32;

    let mut adj = vec![vec![-1; n as usize]; n as usize];
    for e in &edges {
        adj[e[0] as usize][e[1] as usize] = e[2];
        adj[e[1] as usize][e[0] as usize] = e[2];
    }

    let mut heap = BinaryHeap::new();
    for &p in &points {
        heap.push(Reverse((0, p as usize, p_len as usize, vec![p as usize]))); // weight, from, remain, current_path
    }

    let mut ans = vec![];
    let mut visited = vec![vec![vec![false; n as usize]; n as usize]; p_len as usize + 1];

    while let Some(Reverse((d1, from, remain, cp))) = heap.pop() {
        if cp.len() > 4 && cp[cp.len() - 4] == from {
            continue;
        }

        if remain == 0 {
            ans = cp;
            break;
        }

        for (to, &w) in adj[from].iter().enumerate() {
            if w == -1 {
                continue;
            }

            // if !visited[remain][from][to] {
            // visited[remain][from][to] = true;
            let mut cp2 = cp.clone();
            cp2.push(to);

            if points.contains(&(to as i32)) && !cp.contains(&to) {
                heap.push(Reverse((d1 + w, to, remain - 1, cp2)));
            } else {
                heap.push(Reverse((d1 + w, to, remain, cp2)));
            }
            // }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test505() {
        println!("{:?}", sort_stack(vec![4, 1, 3, 2, 5, 0, 3]));
    }

    #[test]
    fn test506() {
        println!(
            "{:?}",
            minimum_cost_with_points_of_interest(
                5,
                vec![
                    vec![0, 1, 4],
                    vec![2, 1, 3],
                    vec![1, 4, 11],
                    vec![3, 2, 3],
                    vec![3, 4, 200]
                ],
                vec![0, 3, 4]
            )
        );
    }
}
