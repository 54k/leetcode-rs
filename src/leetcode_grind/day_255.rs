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

// https://leetcode.com/problems/parallel-courses-ii/description/
pub fn min_number_of_semesters_tle(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut adj = vec![vec![]; n as usize];
    let mut in_degree = vec![0; n as usize];

    for r in relations {
        adj[r[0] as usize - 1].push(r[1] as usize - 1);
        in_degree[r[1] as usize] += 1;
    }

    fn get_candidates(in_degree: &Vec<i32>, taken: &Vec<usize>, n: usize) -> Vec<usize> {
        let mut candidates = Vec::new();
        for i in 0..n {
            if in_degree[i] == 0 && !taken.contains(&i) {
                candidates.push(i);
            }
        }
        candidates
    }

    fn backtrack(
        taken: Vec<usize>,
        in_degree: Vec<i32>,
        adj: &Vec<Vec<usize>>,
        k: usize,
        n: usize,
        memo: &mut HashMap<Vec<usize>, i32>,
    ) -> i32 {
        // println!("taken {:?} degrees {:?}", taken, in_degree);
        if taken.len() == n {
            return 0;
        }
        let key = taken.clone();

        if memo.contains_key(&key) {
            return memo[&key];
        }

        let mut ans = i32::MAX;
        let candidates = get_candidates(&in_degree, &taken, n);

        if candidates.len() == 0 {
            panic!("oops");
        }

        // println!("candidates {:?}", candidates);

        for subset in 1..1 << candidates.len() {
            let ones = (subset as u32).count_ones();
            if ones != (k as u32).min(candidates.len() as u32) {
                continue;
            }

            let mut new_taken = vec![];
            let mut new_in_degree = in_degree.clone();

            for i in 0..candidates.len() {
                if ((subset >> i) & 1) == 1 {
                    new_taken.push(candidates[i]);
                }
            }

            for &i in &new_taken {
                for &neighbor in &adj[i] {
                    new_in_degree[neighbor] -= 1;
                }
            }

            new_taken.extend(taken.clone());

            let next_ans = backtrack(new_taken, new_in_degree, adj, k, n, memo) + 1;
            ans = ans.min(next_ans);
        }

        memo.insert(key, ans);

        ans
    }

    use std::collections::HashMap;
    backtrack(
        vec![],
        in_degree,
        &adj,
        k as usize,
        n as usize,
        &mut HashMap::new(),
    )
}

pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut adj = vec![vec![]; n as usize];
    for r in relations {
        let (from, to) = (r[0] as usize - 1, r[1] as usize - 1);
        adj[from].push(to);
    }
    let mut dp = vec![-1; 1 << n as usize];
    return solve(0, n as usize, k as usize, &adj, &mut dp);

    fn solve(mask: usize, n: usize, k: usize, adj: &Vec<Vec<usize>>, dp: &mut Vec<i32>) -> i32 {
        if mask == (1 << n) - 1 {
            // All courses are taken
            return 0;
        }
        if dp[mask] != -1 {
            // memoization
            return dp[mask];
        }

        let mut in_degree = vec![0; n];

        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                continue;
            }
            for &it in &adj[i] {
                in_degree[it] += 1;
            }
        }

        let mut temp = 0; // For a mask of all nodes with 0-indegree
        for i in 0..n {
            if in_degree[i] == 0 && (mask & (1 << i)) == 0 {
                temp |= 1 << i;
            }
        }

        let mut j = temp;
        let mut cnt = (j as u32).count_ones() as usize; // count of nodes with 0-indegree

        let mut ans = n as i32 + 1; // ans will be 'n' in the worst case, so take (n+1) as infinity

        if cnt > k {
            while j != 0 {
                // iterate through all submasks of temp
                cnt = (j as u32).count_ones() as usize;
                if cnt != k {
                    j = (j - 1) & temp;
                    continue;
                }
                ans = ans.min(1 + solve(mask | j, n, k, adj, dp));
                j = (j - 1) & temp;
            }
        } else {
            ans = ans.min(1 + solve(mask | j, n, k, adj, dp));
        }
        dp[mask] = ans;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semesters() {
        println!("{:?}", min_number_of_semesters(11, vec![], 2));
        println!("{:?}", min_number_of_semesters(14, vec![vec![11, 7]], 2));
        println!(
            "{:?}",
            min_number_of_semesters(5, vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]], 2)
        );
        println!(
            "{:?}",
            min_number_of_semesters(4, vec![vec![2, 1], vec![3, 1], vec![1, 4]], 2)
        );
    }
}
