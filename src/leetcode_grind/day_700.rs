// https://leetcode.com/problems/the-maze-ii/description/
pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    let m = maze.len();
    let n = maze[0].len();

    let mut distance = vec![vec![i32::MAX; n]; m];
    distance[start[0] as usize][start[1] as usize] = 0;

    let mut dirs = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let s = queue.pop_front().unwrap();
        for dir in &dirs {
            let mut x = s[0] + dir.0;
            let mut y = s[1] + dir.1;

            let mut count = 0;
            while x >= 0
                && y >= 0
                && x < m as i32
                && y < n as i32
                && maze[x as usize][y as usize] == 0
            {
                x += dir.0;
                y += dir.1;
                count += 1;
            }

            if distance[s[0] as usize][s[1] as usize] + count
                < distance[(x - dir.0) as usize][(y - dir.1) as usize]
            {
                distance[(x - dir.0) as usize][(y - dir.1) as usize] =
                    distance[s[0] as usize][s[1] as usize] + count;
                queue.push_back(vec![x - dir.0, y - dir.1]);
            }
        }
    }

    if distance[destination[0] as usize][destination[1] as usize] == i32::MAX {
        -1
    } else {
        distance[destination[0] as usize][destination[1] as usize]
    }
}
