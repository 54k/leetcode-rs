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

// https://leetcode.com/problems/sort-colors/
pub fn sort_colors(nums: &mut Vec<i32>) {
    fn n2(nums: &mut Vec<i32>) {
        let mut i = 0;
        'op: while i < nums.len() {
            for j in i..nums.len() {
                if nums[j] < nums[i] {
                    nums.swap(i, j);
                    continue 'op;
                }
            }
            i += 1;
        }
    }

    // https://leetcode.com/problems/sort-colors/solutions/849331/rust-one-pass/
    fn n(nums: &mut Vec<i32>) {
        /*
        The idea is iterating through to push 0s and 2s to sides
        Indices i0 and i2 mark the boundary so that
            0s over [0, i0)
            1s over [i0, i2)
            2s over [i2, len)
        */
        let (mut i, mut i1, mut i2) = (0, 0, nums.len());
        while i < i2 {
            match nums[i] {
                0 => {
                    nums.swap(i1, i);
                    i1 += 1;
                    i += 1;
                }
                2 => {
                    i2 -= 1;
                    nums.swap(i2, i);
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    n(nums)
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

    #[test]
    fn test219() {
        let mut v1 = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut v1);
        println!("{:?}", v1); // [0,0,1,1,2,2]
        let mut v2 = vec![2, 0, 1];
        sort_colors(&mut v2);
        println!("{:?}", v2); // [0,1,2]
    }
}
