// https://leetcode.com/problems/shortest-path-to-get-all-keys/
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};

    let grid = grid
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let (m, n) = (grid.len(), grid[0].len());

    let mut key_set = HashSet::new();
    let mut lock_set = HashSet::new();

    let mut all_keys = 0;
    let (mut start_row, mut start_col) = (-1, -1);

    for i in 0..m {
        for j in 0..n {
            let cell = grid[i][j];

            if cell >= 'a' && cell <= 'f' {
                all_keys += (1 << (cell as i32 - 'a' as i32));
                key_set.insert(cell);
            }

            if cell >= 'A' && cell <= 'F' {
                lock_set.insert(cell);
            }

            if cell == '@' {
                start_row = i as i32;
                start_col = j as i32;
            }
        }
    }

    let mut seen: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col, 0, 0)); // [row, col, key_state, distance]
    seen.insert(0, HashSet::new());
    seen.get_mut(&0).unwrap().insert((start_row, start_col));

    while let Some((cur_r, cur_c, keys, dist)) = queue.pop_front() {
        for d in &DIR {
            let (new_r, new_c) = (cur_r + d.0, cur_c + d.1);

            if 0 <= new_r
                && new_r < m as i32
                && 0 <= new_c
                && new_c < n as i32
                && grid[new_r as usize][new_c as usize] != '#'
            {
                let cell = grid[new_r as usize][new_c as usize];
                // if it is a key
                if key_set.contains(&cell) {
                    // If we have collected it before, no need to revisit this cell.
                    if (1 << (cell as i32 - 'a' as i32)) & keys != 0 {
                        continue;
                    }
                    // Otherwise, we can walk to this cell and pick it up.
                    let new_keys = keys | (1 << (cell as i32 - 'a' as i32));

                    // If we collect all keys, return dist + 1.
                    // Otherwise, just add this state to seen and queue.
                    if new_keys == all_keys {
                        return dist + 1;
                    }
                    seen.entry(new_keys).or_insert(HashSet::new());
                    seen.get_mut(&new_keys).unwrap().insert((new_r, new_c));
                    queue.push_back((new_r, new_c, new_keys, dist + 1));
                }

                // If it is a lock and we don't have its key, continue.
                if lock_set.contains(&cell) && (keys & (1 << (cell as i32 - 'A' as i32))) == 0 {
                    continue;
                }
                // We can walk to this cell if we haven't been here before with the same key state.
                else if !seen.get(&keys).unwrap().contains(&(new_r, new_c)) {
                    seen.get_mut(&keys).unwrap().insert((new_r, new_c));
                    queue.push_back((new_r, new_c, keys, dist + 1));
                }
            }
        }
    }

    -1
}

// https://leetcode.com/problems/minimum-cost-to-merge-stones/description/
// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/247567/java-c-python-dp/
// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/1432667/explained-to-make-you-visualise-the-solution-detailed-explanation/
// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/675912/dp-code-decoded-for-non-experts-like-me/
pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    // Time: O(N^3/K), Space: O(N^2)

    // Fesibility check:
    // Let the final length of the stones after m merges is 1.
    // Each merge reduces the length by K-1.
    // After first merge, length is N - (K-1), after second, length is N - (K-1) - (K-1) = N- 2*(K-1).
    // After m merges, length = m*(K-1) which is 1.
    // That is, m(K-1) = N - 1 to be able to get to 1.
    // Re arranging, N-1/K-1 needs to an integer, m for given N and K.
    // Thus the check, (N-1) % (K-1) == 0

    // Populating the table:
    // We start from a minimum length, len of K. Anything lesser,
    // does not incur any cost.
    // We then find minimum costs for all sub-problems of lengths from K to N.
    // Every time, we solve a sub-problem for which, len-1 is a multiple of K-1, it signifies a real merge.
    // (eg. For N = 5 & K = 3, len of 3 and 5 are real merges, whereas len of 4 is not!).
    // The reason to calculate sub-problems which are not real merges,
    // is to copy over the minimum of previous merge costs towards caculating the next real merge (merely bookkeeping).

    // Cost calculation and Partial sum table
    // Every time len-1 is a multiple of K, we incur a cost.
    // This cost = sum of ( all stones under the span of len).
    // Since the cost is merely sum of a contiguos sub-array of input,
    // we pre-caculate the partial sums for O(1) lookup. The check (len-1 )% (K-1) == 0,
    // signals a true merge and we calcuate the additional costs for this sub-problem.

    // Trickiest part for me
    // Indexing over the previous sub-problems to calculate the current problem, is at the heart of this question.
    // This current problem is composed of pairs of sub-problems of sizes ( [1, len-1], [K, rest], [1+ K-1 + K-1, rest], ... [len-1, 1] )
    // which is equivalent to ( [1, len-1], [1 + (K-1), rest], [1 + 2(K-1), rest] ...[len-1, 1] ).
    // We can add the cost of the merge only after finding the min of the previous sub-problem costs and it affects all the candidates for the DP entry equally.

    // Work through these examples for clarity
    // So, the first example is a trick! Just for K = 2, every value of len Несет a cost! This threw me off for some time. Here is an example that helped me clear up the nasty bugs:
    // [1,4,3,3,2], K = 3, 4, 5
    fn top_down_2d(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        if (n as i32 - 1) % (k - 1) > 0 {
            return -1;
        }

        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + stones[i];
        }

        let mut dp = vec![vec![0; n]; n];

        for m in k..=n as i32 {
            for i in 0..=n as i32 - m {
                let j = i + m - 1;
                dp[i as usize][j as usize] = i32::MAX;
                let mut mid = i;
                while mid < j {
                    dp[i as usize][j as usize] = dp[i as usize][j as usize]
                        .min(dp[i as usize][mid as usize] + dp[mid as usize + 1][j as usize]);
                    mid += k - 1;
                }

                if (j - i) % (k - 1) == 0 {
                    dp[i as usize][j as usize] += prefix[j as usize + 1] - prefix[i as usize];
                }
            }
        }

        dp[0][n - 1]
    }

    top_down_2d(stones, k)
}

// https://leetcode.com/problems/cracking-the-safe/description/
pub fn crack_safe(n: i32, k: i32) -> String {
    todo!()
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-v/description/
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vi/description/
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vii/description/
pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            todo!()
        }

        fn update(&self, row: i32, col: i32, val: i32) {
            todo!()
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            todo!()
        }
    }
}

// https://leetcode.com/problems/maximize-grid-happiness/description/
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    todo!()
}
