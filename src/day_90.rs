// https://leetcode.com/problems/as-far-from-land-as-possible/description/
pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    // Multi-source BFS
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    use std::collections::VecDeque;
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![0; m]; n];

    let mut queue = VecDeque::new();

    for i in 0..n {
        for j in 0..m {
            visited[i][j] = grid[i][j];
            if grid[i][j] == 1 {
                queue.push_back((i as i32, j as i32))
            }
        }
    }
    let mut distance = -1;
    while !queue.is_empty() {
        let mut q_size = queue.len();
        while q_size > 0 {
            let (i, j) = queue.pop_front().unwrap();
            for (x, y) in DIR {
                let i = i + x;
                let j = j + y;
                if i >= 0
                    && i < m as i32
                    && j >= 0
                    && j < n as i32
                    && visited[i as usize][j as usize] == 0
                {
                    visited[i as usize][j as usize] = 1;
                    queue.push_back((i, j));
                }
            }
            q_size -= 1;
        }
        // After each iteration of queue elements, increment distance
        // as we covered 1 unit distance from all cells in every direction.
        distance += 1;
    }
    if distance == 0 {
        -1
    } else {
        distance
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test218() {
        println!(
            "{}",
            max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]])
        ); // 2
        println!(
            "{}",
            max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]])
        ); // 4
    }
}
