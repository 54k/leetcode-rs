// https://leetcode.com/problems/snakes-and-ladders/solutions/2912646/snakes-and-ladders/?orderBy=most_relevant
pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    use std::collections::*;

    let n = board.len();
    let mut label = 1;
    let mut cells = vec![(0, 0); n * n + 1];
    let mut columns = vec![0; n];
    for i in 0..n {
        columns[i] = i;
    }
    for row in (0..n).rev() {
        for &col in columns.iter() {
            cells[label] = (row, col);
            label += 1;
        }
        columns.reverse();
    }

    let mut dist = vec![-1; n * n + 1];
    dist[1] = 0;
    let mut q = VecDeque::new();
    q.push_back(1);

    while let Some(curr) = q.pop_front() {
        for next in curr + 1..=(curr + 6).min(n * n) {
            let row = cells[next].0;
            let col = cells[next].1;
            let destination = if board[row][col] != -1 {
                board[row][col] as usize
            } else {
                next as usize
            };
            if dist[destination] == -1 {
                dist[destination] = dist[curr] + 1;
                q.push_back(destination);
            }
        }
    }

    dist[n * n]
}

// https://leetcode.com/problems/edit-distance/solutions/3040251/edit-distance/?orderBy=most_relevant
pub fn min_distance(word1: String, word2: String) -> i32 {
    fn rec(word1: String, word2: String) -> i32 {
        const INF: i32 = 100000007;
        let n = word1.len();
        let m = word2.len();
        let mut d = vec![vec![INF; m + 1]; n + 1];

        let a = word1.chars().collect::<Vec<_>>();
        let b = word2.chars().collect::<Vec<_>>();

        fn go(a: &Vec<char>, b: &Vec<char>, d: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
            if d[i][j] == INF {
                match (i, j) {
                    (_, 0) => {
                        d[i][0] = i as i32;
                    }
                    (0, _) => {
                        d[0][j] = j as i32;
                    }
                    (i, j) => {
                        let ins = go(a, b, d, i - 1, j) + 1;
                        let del = go(a, b, d, i, j - 1) + 1;
                        let sub = go(a, b, d, i - 1, j - 1)
                            + (a[i - 1] as i32 - b[j - 1] as i32).abs().min(1);
                        d[i][j] = (ins).min(del).min(sub)
                    }
                };
            }
            d[i][j]
        }

        go(&a, &b, &mut d, n, m)
    }

    fn dp(word1: String, word2: String) -> i32 {
        const INF: i32 = 100000007;
        let n = word1.len();
        let m = word2.len();
        let mut d = vec![vec![INF; m + 1]; n + 1];

        let a = word1.chars().collect::<Vec<_>>();
        let b = word2.chars().collect::<Vec<_>>();

        d[0][0] = 0;
        for i in 0..=n {
            d[i][0] = i as i32;
        }
        for j in 0..=m {
            d[0][j] = j as i32;
        }

        for i in 1..=n {
            for j in 1..=m {
                if a[i - 1] == b[j - 1] {
                    d[i][j] = d[i - 1][j - 1];
                } else {
                    d[i][j] = d[i - 1][j].min(d[i][j - 1]).min(d[i - 1][j - 1]) + 1;
                }
            }
        }
        d[n][m]
    }
    dp(word1, word2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test174() {
        println!(
            "{}",
            snakes_and_ladders(vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1],
            ])
        ); // 4
    }

    #[test]
    fn test175() {
        println!(
            "{}",
            min_distance("intention".to_string(), "execution".to_string())
        ); // 5
    }
}
