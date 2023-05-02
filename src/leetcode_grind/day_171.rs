use std::sync::Arc;

// https://leetcode.com/problems/sign-of-the-product-of-an-array/
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut min = 0;
    for num in nums {
        if num < 0 {
            min += 1;
        } else if num == 0 {
            return 0;
        }
    }
    if min % 2 == 1 {
        -1
    } else {
        1
    }
}

// https://leetcode.com/problems/walls-and-gates/description/
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    use std::collections::VecDeque;
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q = VecDeque::new();
    for i in 0..rooms.len() {
        for j in 0..rooms[i].len() {
            if rooms[i][j] == 0 {
                q.push_back((i as i32, j as i32));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        for dir in DIRS {
            let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);
            if x < 0
                || x >= rooms.len() as i32
                || y < 0
                || y >= rooms[0].len() as i32
                || rooms[x as usize][y as usize] != i32::MAX
            {
                continue;
            }
            rooms[x as usize][y as usize] = rooms[i as usize][j as usize] + 1;
            q.push_back((x, y));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test474() {
        println!("{}", array_sign(vec![-1, 1, -1, 1, -1]));
    }

    #[test]
    fn test475() {
        let mut rooms = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        walls_and_gates(&mut rooms);
        println!("{:?}", rooms);
    }
}
