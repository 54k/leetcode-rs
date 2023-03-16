#[allow(dead_code)]
pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(x: i32, y: i32, g: &mut Vec<Vec<i32>>, walked: i32, walks: i32) -> i32 {
        if x < 0
            || y < 0
            || x >= g.len() as i32
            || y >= g[0].len() as i32
            || g[x as usize][y as usize] < 0
        {
            return 0;
        }

        if g[x as usize][y as usize] == 2 {
            return if walked == walks { 1 } else { 0 };
        }

        g[x as usize][y as usize] = -1;
        let ans = dfs(x + 1, y, g, walked + 1, walks)
            + dfs(x - 1, y, g, walked + 1, walks)
            + dfs(x, y + 1, g, walked + 1, walks)
            + dfs(x, y - 1, g, walked + 1, walks);
        g[x as usize][y as usize] = 0;
        ans
    }

    let n = grid.len();
    let m = grid[0].len();

    let mut sx = 0;
    let mut sy = 0;
    let mut walks = 1;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                walks += 1;
            }
            if grid[i][j] == 1 {
                sx = i as i32;
                sy = j as i32;
            }
        }
    }

    dfs(sx, sy, &mut grid, 0, walks)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test136() {
        println!(
            "{}",
            unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]])
        );
    }
}
