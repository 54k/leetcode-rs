// https://leetcode.com/problems/number-of-closed-islands/description/
pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs_approach(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        fn dfs((x, y): (i32, i32), grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
            if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                return false;
            }
            if grid[x as usize][y as usize] == 1 || visited[x as usize][y as usize] {
                return true;
            }
            visited[x as usize][y as usize] = true;
            let mut is_closed = true;
            for i in 0..DIRS.len() {
                let r = x + DIRS[i].0;
                let c = y + DIRS[i].1;
                if !dfs((r, c), grid, visited) {
                    is_closed = false;
                }
            }
            is_closed
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut c_count = 0;
        for i in 0..grid.len() as i32 {
            for j in 0..grid[0].len() as i32 {
                if grid[i as usize][j as usize] == 0
                    && !visited[i as usize][j as usize]
                    && dfs((i, j), &grid, &mut visited)
                {
                    c_count += 1;
                }
            }
        }
        c_count
    }
    fn bfs_approach(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        fn bfs((x, y): (i32, i32), grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            visited[x as usize][y as usize] = true;
            let mut is_closed = true;
            while let Some((x, y)) = queue.pop_front() {
                for i in 0..DIRS.len() {
                    let r = x + DIRS[i].0;
                    let c = y + DIRS[i].1;
                    if r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32 {
                        return false;
                    } else if grid[r as usize][c as usize] == 0 && !visited[r as usize][c as usize]
                    {
                        visited[r as usize][c as usize] = true;
                        queue.push_back((r, c));
                    }
                }
            }
            is_closed
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut c_count = 0;
        for i in 0..grid.len() as i32 {
            for j in 0..grid[0].len() as i32 {
                if grid[i as usize][j as usize] == 0
                    && !visited[i as usize][j as usize]
                    && bfs((i, j), &grid, &mut visited)
                {
                    c_count += 1;
                }
            }
        }
        c_count
    }
    bfs_approach(grid)
}

// https://leetcode.com/problems/maximum-number-of-balloons/description/
// https://leetcode.com/problems/maximum-number-of-balloons/editorial/
pub fn max_number_of_balloons(text: String) -> i32 {
    let mut freq = vec![0; 26];
    for ch in text.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    let mut ans = i32::MAX;
    for x in "balon".chars() {
        let f = freq[x as usize - 'a' as usize];
        if x == 'l' || x == 'o' {
            ans = ans.min(f / 2);
        } else {
            ans = ans.min(f);
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/
pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = i32::MAX;
    for i in 0..cards.len() {
        let num = cards[i];
        if map.contains_key(&num) {
            ans = ans.min(i as i32 - map[&num] as i32 + 1)
        }
        map.insert(num, i);
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    fn digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = -1;
    for num in nums {
        let digit_sum = digit_sum(num);
        if map.contains_key(&digit_sum) {
            ans = ans.max(map[&digit_sum] + num);
        }
        if *map.get(&digit_sum).unwrap_or(&0) < num {
            map.insert(digit_sum, num);
        }
    }
    ans
}

// https://leetcode.com/problems/equal-row-and-column-pairs/
pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut dict1 = HashMap::new();
    for i in 0..grid.len() {
        *dict1.entry(grid[i].clone()).or_insert(0) += 1;
    }
    let mut dict2 = HashMap::new();
    for i in 0..grid[0].len() {
        let mut col = vec![0; grid.len()];
        for j in 0..grid.len() {
            col[j] = grid[j][i];
        }
        *dict2.entry(col).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k, v) in dict1 {
        if dict2.contains_key(&k) {
            ans += dict2[&k] * v
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test406() {
        println!(
            "{}",
            closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0]
            ])
        ); // 2
    }

    #[test]
    fn test407() {
        println!("{}", max_number_of_balloons("nlaebolko".to_string())); // 1
        println!("{}", max_number_of_balloons("loonbalxballpoon".to_string())); // 2
    }

    #[test]
    fn test408() {
        println!("{}", minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]));
    }

    #[test]
    fn test409() {
        println!("{}", maximum_sum(vec![18, 43, 36, 13, 7])); // 54
        println!(
            "{}",
            maximum_sum(vec![
                279, 169, 463, 252, 94, 455, 423, 315, 288, 64, 494, 337, 409, 283, 283, 477, 248,
                8, 89, 166, 188, 186, 128
            ])
        ); // 872
    }

    #[test]
    fn test410() {
        println!(
            "{}",
            equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ])
        ); // 3
    }
}
