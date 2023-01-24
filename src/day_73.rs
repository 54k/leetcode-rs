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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test172() {
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
}
