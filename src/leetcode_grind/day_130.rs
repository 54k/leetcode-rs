// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/editorial/
pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    fn create_adj(n: i32, roads: &Vec<Vec<i32>>) -> (Vec<Vec<(usize, i32)>>, Vec<bool>) {
        let mut adj = vec![vec![]; n as usize + 1];
        let visited = vec![false; n as usize + 1];
        for road in roads {
            let from = road[0] as usize;
            let to = road[1] as usize;
            let weight = road[2];
            adj[from].push((to, weight));
            adj[to].push((from, weight));
        }
        (adj, visited)
    }
    fn using_bfs(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let (adj, mut visited) = create_adj(n, &roads);
        let mut queue = VecDeque::new();
        let mut ans = i32::MAX;

        queue.push_back(1usize);
        visited[1] = true;

        while let Some(v) = queue.pop_front() {
            if adj[v].is_empty() {
                continue;
            }

            for (u, w) in &adj[v] {
                ans = ans.min(*w);
                if !visited[*u] {
                    visited[*u] = true;
                    queue.push_back(*u);
                }
            }
        }
        ans
    }
    fn using_dfs(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let (adj, mut visited) = create_adj(n, &roads);
        fn dfs(v: usize, adj: &Vec<Vec<(usize, i32)>>, visited: &mut Vec<bool>, ans: &mut i32) {
            visited[v] = true;
            for (u, w) in &adj[v] {
                *ans = (*ans).min(*w);
                if !visited[*u] {
                    dfs(*u, adj, visited, ans);
                }
            }
        }
        let mut ans = i32::MAX;
        dfs(1, &adj, &mut visited, &mut ans);
        ans
    }
    using_dfs(n, roads)
}

// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/description/
pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    nums.sort();
    let mut freq = nums.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    for num in nums {
        if *freq.get(&num).unwrap() == 0 {
            continue;
        }
        for i in 0..k {
            if freq.contains_key(&(num + i)) {
                *freq.get_mut(&(num + i)).unwrap() -= 1;
            } else {
                return false;
            }
        }
    }
    true
}

// https://leetcode.com/problems/hand-of-straights/
pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    use std::collections::HashMap;
    hand.sort();
    let mut freq = hand.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    for h in hand {
        if *freq.get(&h).unwrap() == 0 {
            continue;
        }
        for i in 0..group_size {
            if freq.contains_key(&(h + i)) && *freq.get(&(h + i)).unwrap() > 0 {
                *freq.get_mut(&(h + i)).unwrap() -= 1;
            } else {
                return false;
            }
        }
    }
    true
}

// https://leetcode.com/problems/all-divisions-with-the-highest-score-of-a-binary-array/description/
pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    let mut onex_prefix = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        if nums[i] == 1 {
            onex_prefix[i] += 1;
        }
        if i < nums.len() - 1 {
            onex_prefix[i] += onex_prefix[i + 1];
        }
    }

    let mut zeros_count = 0;
    let mut scores = vec![0; nums.len() + 1];
    let mut max_score = 0;
    for i in 0..=nums.len() {
        if i == nums.len() {
            scores[i] = zeros_count;
        } else {
            scores[i] = (zeros_count + onex_prefix[i]);
            if nums[i] == 0 {
                zeros_count += 1;
            }
        }
        max_score = max_score.max(scores[i]);
    }
    scores[nums.len()] = zeros_count;

    scores
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x == max_score)
        .map(|(i, _)| i as i32)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test362() {
        println!(
            "{}",
            min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            )
        ); // 5
    }

    #[test]
    fn test_is_possible_divide() {
        println!("{}", is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4)); // true
        println!(
            "{}",
            is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3)
        ); // true
        println!("{}", is_possible_divide(vec![1, 2, 3, 4], 3)); // false
    }

    #[test]
    fn test_is_n_straight_hand() {
        println!("{}", is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)); // true
        println!("{}", is_n_straight_hand(vec![1, 2, 3, 4, 5], 4)); // false
        println!(
            "{}",
            is_n_straight_hand(vec![3, 4, 7, 4, 6, 3, 6, 5, 2, 8], 2)
        ); // false
    }

    #[test]
    fn test_max_score_indices() {
        println!("{:?}", max_score_indices(vec![0, 0, 1, 0])); // [2, 4]
        println!("{:?}", max_score_indices(vec![0, 0, 0])); // [3]
    }
}
