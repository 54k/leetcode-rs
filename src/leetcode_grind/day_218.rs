// https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/description/
pub fn remove_digit(number: String, digit: char) -> String {
    let mut ans = String::new();
    let number = number.chars().collect::<Vec<_>>();
    for i in 0..number.len() {
        if number[i] == digit {
            let s = format!(
                "{}{}",
                number[..i].iter().copied().collect::<String>(),
                number[i + 1..].iter().copied().collect::<String>()
            );
            ans = ans.max(s);
        }
    }
    ans
}

// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/
pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
    pub fn using_tabulation(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const MOD: i32 = 1000000007;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut dp = vec![vec![1; n]; m];

        let mut cell_list = vec![vec![0; 2]; m * n];
        for i in 0..m {
            for j in 0..n {
                let index = i * n + j;
                cell_list[index][0] = i;
                cell_list[index][1] = j;
            }
        }
        cell_list.sort_by(|a, b| grid[a[0]][a[1]].cmp(&grid[b[0]][b[1]]));

        for cell in cell_list {
            let (i, j) = (cell[0], cell[1]);

            for d in &directions {
                let (curr_i, curr_j) = (i as i32 + d.0, j as i32 + d.1);
                if 0 <= curr_i
                    && (curr_i as usize) < m
                    && 0 <= curr_j
                    && (curr_j as usize) < n
                    && grid[curr_i as usize][curr_j as usize] > grid[i][j]
                {
                    dp[curr_i as usize][curr_j as usize] += dp[i][j];
                    dp[curr_i as usize][curr_j as usize] %= MOD;
                }
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans += dp[i][j];
                ans %= MOD;
            }
        }
        ans
    }

    pub fn using_dfs(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const MOD: i32 = 1000000007;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut dp = vec![vec![0; n]; m];

        fn dfs(
            i: i32,
            j: i32,
            grid: &Vec<Vec<i32>>,
            dp: &mut Vec<Vec<i32>>,
            directions: &Vec<(i32, i32)>,
        ) -> i32 {
            if dp[i as usize][j as usize] != 0 {
                return dp[i as usize][j as usize];
            }

            let mut ans = 1;

            for d in directions {
                let (prev_i, prev_j) = (i + d.0, j + d.1);

                if 0 <= prev_i
                    && prev_i < grid.len() as i32
                    && 0 <= prev_j
                    && prev_j < grid[0].len() as i32
                    && grid[prev_i as usize][prev_j as usize] < grid[i as usize][j as usize]
                {
                    ans += dfs(prev_i, prev_j, grid, dp, directions);
                    ans %= MOD;
                }
            }
            dp[i as usize][j as usize] = ans;
            ans
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans = (ans + dfs(i as i32, j as i32, &grid, &mut dp, &directions)) % MOD;
            }
        }
        ans
    }
    using_dfs(grid)
}

// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/description/
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    fn dfs(grid: &Vec<Vec<i32>>, i: i32, j: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i as usize][j as usize] != 0 {
            return dp[i as usize][j as usize];
        }
        let mut ans = 1;
        for d in DIR {
            let (n_i, n_j) = (d.0 + i, d.1 + j);

            if 0 <= n_i
                && n_i < grid.len() as i32
                && 0 <= n_j
                && n_j < grid[0].len() as i32
                && grid[n_i as usize][n_j as usize] > grid[i as usize][j as usize]
            {
                ans = ans.max(1 + dfs(grid, n_i, n_j, dp));
            }
        }
        dp[i as usize][j as usize] = ans;
        ans
    }
    let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
    let mut ans = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            ans = ans.max(dfs(&matrix, i as i32, j as i32, &mut dp));
        }
    }
    ans
}

// https://leetcode.com/problems/maximum-strictly-increasing-cells-in-a-matrix/description/
pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
    pub fn brute_tle(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut cells = vec![vec![0; 2]; m * n];
        for i in 0..m {
            for j in 0..n {
                let index = i * n + j;
                cells[index][0] = i;
                cells[index][1] = j;
            }
        }
        cells.sort_by(|a, b| mat[a[0]][a[1]].cmp(&mat[b[0]][b[1]]));

        let mut ans = 0;
        let mut dp = vec![vec![1; n]; m];
        for cell in cells {
            let (i, j) = (cell[0], cell[1]);

            // row
            for k in 0..m {
                if mat[k][j] < mat[i][j] {
                    dp[i][j] = dp[i][j].max(1 + dp[k][j]);
                }
            }

            // col
            for k in 0..n {
                if mat[i][k] < mat[i][j] {
                    dp[i][j] = dp[i][j].max(1 + dp[i][k]);
                }
            }

            ans = ans.max(dp[i][j]);
        }
        ans
    }

    pub fn btree_map(mat: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeMap;
        let mut cells = BTreeMap::new();
        let (m, n) = (mat.len(), mat[0].len());
        for i in 0..m {
            for j in 0..n {
                cells.entry(mat[i][j]).or_insert(vec![]).push(vec![i, j]);
            }
        }
        let mut max_vals = vec![0; m + n];
        let mut ans = 0;
        let mut dp = vec![vec![0; n]; m];
        for val in cells.keys() {
            for cell in &cells[val] {
                let (i, j) = (cell[0], cell[1]);
                dp[i][j] = max_vals[i].max(max_vals[j + m]) + 1;
                ans = ans.max(dp[i][j]);
            }

            for cell in &cells[val] {
                let (i, j) = (cell[0], cell[1]);
                max_vals[i] = max_vals[i].max(dp[i][j]);
                max_vals[j + m] = max_vals[j + m].max(dp[i][j]);
            }
        }
        ans
    }

    btree_map(mat)
}

// https://leetcode.com/problems/total-appeal-of-a-string/description/
pub fn appeal_sum(s: String) -> i64 {
    // public long appealSum(String s) {
    //     int[] a = new int[26];
    //     long c=0;
    //     long ans=0;
    //     for(int i=0;i<s.length();i++)
    //     {
    //         c-=a[s.charAt(i)-'a'];
    //         ans+=c+=a[s.charAt(i)-'a']=i+1;
    //     }
    //     return ans;
    // }
    let s = s.chars().collect::<Vec<_>>();
    let mut idx = vec![-1; 26];
    let (mut x, mut y) = (1, 1);
    idx[s[0] as usize - 'a' as usize] = 1;
    for (i, ch) in s.into_iter().enumerate().skip(1) {
        let chi = ch as usize - 'a' as usize;

        x += i as i32 + 1; // ans till ith idx

        if idx[chi] > 0 {
            x -= idx[chi];
        }

        y += x as i64;

        idx[chi] = i as i32 + 1;
    }

    y
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
