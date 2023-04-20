// https://leetcode.com/problems/path-with-minimum-effort/description/
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    fn dfs(heights: &Vec<Vec<i32>>, effort: i32) -> bool {
        const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut seen = vec![vec![false; heights[0].len()]; heights.len()];
        let mut stack = vec![];
        stack.push((0, 0));
        seen[0][0] = true;
        while let Some((x, y)) = stack.pop() {
            if x == heights.len() as i32 - 1 && y == heights[0].len() as i32 - 1 {
                return true;
            }
            for d in DIR {
                let (nx, ny) = (x + d.0, y + d.1);
                if nx >= 0
                    && nx < heights.len() as i32
                    && ny >= 0
                    && ny < heights[0].len() as i32
                    && !seen[nx as usize][ny as usize]
                    && effort
                        >= (heights[nx as usize][ny as usize] - heights[x as usize][y as usize])
                            .abs()
                {
                    seen[nx as usize][ny as usize] = true;
                    stack.push((nx, ny));
                }
            }
        }
        false
    }

    let mut lo = 0;
    let mut hi = 0;
    for h in &heights {
        for j in h {
            hi = hi.max(*j);
        }
    }

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if dfs(&heights, mid) {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_444() {
        println!(
            "{}",
            minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]])
        ); // 2
    }
}
