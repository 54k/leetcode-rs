// https://leetcode.com/problems/longest-palindromic-substring
pub fn longest_palindrome_dp(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![false; s.len()]; s.len()];

    let (mut lo, mut hi) = (0, 0);
    for i in 0..s.len() {
        dp[i][i] = true;
        if i < s.len() - 1 && s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            lo = i;
            hi = i + 1;
        }
    }

    for diff in 2..s.len() {
        for start in 0..s.len() - diff {
            let end = start + diff;
            if s[start] == s[end] && dp[start + 1][end - 1] {
                dp[start][end] = true;
                if hi - lo < end - start {
                    lo = start;
                    hi = end;
                }
            }
        }
    }

    s[lo..=hi].into_iter().collect()
}

pub fn longest_palindrome_expand_around_center(s: String) -> String {
    fn expand(s: &[char], start: usize, end: usize) -> i32 {
        let mut left = start as i32;
        let mut right = end as i32;

        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        right - left - 1
    }

    let s = s.chars().collect::<Vec<_>>();
    let (mut lo, mut hi): (usize, usize) = (0, 0);

    for center in 0..s.len() {
        let odd_len = expand(&s, center, center);
        if odd_len > (hi - lo + 1) as i32 {
            let dist = odd_len as usize / 2;
            lo = center - dist;
            hi = center + dist;
        }

        let even_len = expand(&s, center, center + 1);
        if even_len > (hi - lo + 1) as i32 {
            let dist = (even_len as usize / 2) - 1;
            lo = center - dist;
            hi = center + dist + 1;
        }
    }

    s[lo..=hi].into_iter().collect()
}

// https://leetcode.com/problems/largest-local-values-in-a-matrix/
pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut max_local = vec![vec![0; n - 2]; n - 2];

    fn get_max_local(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut max = 0;
        for r in i - 1..=i + 1 {
            for c in j - 1..=j + 1 {
                max = max.max(grid[r][c]);
            }
        }
        max
    }

    for cr in 0..max_local.len() {
        for cc in 0..max_local[0].len() {
            max_local[cr][cc] = get_max_local(&grid, cr + 1, cc + 1);
        }
    }

    max_local
}

// https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/description/
pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut rows = HashMap::new();
    let mut cols = HashMap::new();

    fn check(hm: &HashMap<usize, i32>, target: i32) -> bool {
        for val in hm.values() {
            if *val != target {
                return false;
            }
        }
        true
    }

    for r in 0..matrix.len() {
        let mut hs = HashSet::new();
        for c in 0..matrix[0].len() {
            if !hs.insert(matrix[r][c]) {
                return false;
            }
            *rows.entry(r).or_insert(0) += matrix[r][c];
            *cols.entry(c).or_insert(0) += matrix[r][c];
        }
        hs.clear();
    }

    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;

    if check(&rows, n * (n + 1) / 2) && check(&cols, m * (m + 1) / 2) {
        return true;
    }
    false
}

// https://leetcode.com/problems/check-if-matrix-is-x-matrix/description/
pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if i == j || i == grid.len() - 1 - j {
                if grid[i][j] == 0 {
                    return false;
                }
            } else if grid[i][j] != 0 {
                return false;
            }
        }
    }
    true
}

// https://leetcode.com/problems/bomb-enemy/description/
pub fn max_killed_enemies_brute_force(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut max_killed = 0;
    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == '0' {
                let mut killed = 0;

                for i in (0..r).rev() {
                    if grid[i][c] == 'E' {
                        killed += 1;
                    } else if grid[i][c] == 'W' {
                        break;
                    }
                }

                for i in r..m {
                    if grid[i][c] == 'E' {
                        killed += 1;
                    } else if grid[i][c] == 'W' {
                        break;
                    }
                }

                for i in (0..c).rev() {
                    if grid[r][i] == 'E' {
                        killed += 1;
                    } else if grid[r][i] == 'W' {
                        break;
                    }
                }

                for i in c..n {
                    if grid[r][i] == 'E' {
                        killed += 1;
                    } else if grid[r][i] == 'W' {
                        break;
                    }
                }

                max_killed = max_killed.max(killed);
            }
        }
    }

    max_killed
}

pub fn max_killed_enemies_dp(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut max_killed = 0;
    let mut row_hits = 0;
    let mut col_hits = vec![0; n];

    for r in 0..m {
        for c in 0..n {
            if c == 0 || grid[r][c - 1] == 'W' {
                row_hits = 0;
                for k in c..n {
                    if grid[r][k] == 'E' {
                        row_hits += 1;
                    } else if grid[r][k] == 'W' {
                        break;
                    }
                }
            }

            if r == 0 || grid[r - 1][c] == 'W' {
                col_hits[c] = 0;
                for k in r..m {
                    if grid[k][c] == 'E' {
                        col_hits[c] += 1;
                    } else if grid[k][c] == 'W' {
                        break;
                    }
                }
            }

            if grid[r][c] == '0' {
                max_killed = max_killed.max(row_hits + col_hits[c]);
            }
        }
    }
    max_killed
}

// https://leetcode.com/problems/flipping-an-image/
pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = image[0].len();
    let mut image = image;

    for r in 0..image.len() {
        let mut row = &mut image[r];
        for i in 0..(n + 1) / 2 {
            let tmp = row[i] ^ 1;
            row[i] = row[n - 1 - i] ^ 1;
            row[n - 1 - i] = tmp;
        }
    }

    image
}

// https://leetcode.com/problems/delete-greatest-value-in-each-row/
pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut m = grid.len();
    let mut n = grid[0].len();

    let mut ans = 0;

    for (r, row) in grid.iter_mut().enumerate() {
        row.sort();
    }

    let mut c = n as i32 - 1;
    while c >= 0 {
        let mut max = 0;
        for r in 0..m {
            max = max.max(grid[r][c as usize]);
        }
        ans += max;
        c -= 1;
    }

    ans
}

// https://leetcode.com/problems/row-with-maximum-ones/description/
pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut mat = mat;
    let mut rmax = 0;
    let mut count = 0;
    for r in 0..mat.len() {
        for c in (0..mat[0].len() - 1).rev() {
            mat[r][c] += mat[r][c + 1];
        }
        if mat[r][0] > count {
            rmax = r;
            count = mat[r][0];
        }
    }
    return vec![rmax as i32, count];
}
