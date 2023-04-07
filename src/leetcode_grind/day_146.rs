use std::collections::HashMap;

// https://leetcode.com/problems/number-of-enclaves/description/
pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    fn using_dfs(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        fn flood((x, y): (i32, i32), grid: &mut Vec<Vec<i32>>) {
            if x < 0
                || x >= grid.len() as i32
                || y < 0
                || y >= grid[0].len() as i32
                || grid[x as usize][y as usize] == 0
            {
                return;
            }
            grid[x as usize][y as usize] &= 0;
            for (nx, ny) in DIR {
                let nx = x + nx;
                let ny = y + ny;
                flood((nx, ny), grid);
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i * j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 && grid[i][j] == 1 {
                    flood((i as i32, j as i32), &mut grid);
                }
            }
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn using_bfs(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        fn flood((x, y): (i32, i32), grid: &mut Vec<Vec<i32>>) {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            while let Some((x, y)) = queue.pop_front() {
                if x < 0
                    || x >= grid.len() as i32
                    || y < 0
                    || y >= grid[0].len() as i32
                    || grid[x as usize][y as usize] == 0
                {
                    continue;
                }
                grid[x as usize][y as usize] &= 0;
                for (nx, ny) in DIR {
                    let nx = x + nx;
                    let ny = y + ny;
                    queue.push_back((nx, ny));
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i * j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 && grid[i][j] == 1 {
                    flood((i as i32, j as i32), &mut grid);
                }
            }
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
    using_bfs(grid)
}

// https://leetcode.com/problems/binary-subarrays-with-sum/description/
// https://leetcode.com/problems/binary-subarrays-with-sum/editorial/
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    use std::collections::HashMap;
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }

    let mut map = HashMap::new();
    let mut ans = 0;
    for num in prefix {
        ans += *map.get(&num).unwrap_or(&0);
        *map.entry(num + goal).or_insert(0) += 1;
    }
    ans
}

// https://leetcode.com/problems/maximum-erasure-value/
pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut prefix = vec![0; nums.len() + 1];
    let mut set = HashSet::new();
    let mut ans = 0;
    let mut j = 0;
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
        while j < i && set.contains(&nums[i]) {
            set.remove(&nums[j]);
            j += 1;
        }
        set.insert(nums[i]);
        ans = ans.max(prefix[i + 1] - prefix[j]);
    }
    ans
}

// https://leetcode.com/problems/custom-sort-string/description/
pub fn custom_sort_string(order: String, s: String) -> String {
    fn using_sort(order: String, s: String) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, ch) in s.chars().enumerate() {
            map.insert(ch, i);
        }
        for (i, ch) in order.chars().enumerate() {
            map.insert(ch, i);
        }
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_by(|a, b| map[a].cmp(&map[b]));
        s.into_iter().collect()
    }
    fn using_counting(order: String, s: String) -> String {
        let mut count = vec![0; 26];
        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        let mut ans = String::new();
        for ch in order.chars() {
            let mut i = 0;
            while count[ch as usize - 'a' as usize] > i {
                ans.push(ch);
                i += 1;
            }
            count[ch as usize - 'a' as usize] = 0;
        }
        for ch in s.chars() {
            let mut i = 0;
            while count[ch as usize - 'a' as usize] > i {
                ans.push(ch);
                i += 1;
            }
            count[ch as usize - 'a' as usize] = 0;
        }
        ans
    }
    using_counting(order, s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test411() {
        println!(
            "{}",
            num_enclaves(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ])
        ); // 0

        println!(
            "{}",
            num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ])
        ); // 3
    }

    #[test]
    fn test412() {
        println!(
            "{}",
            maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
        ); //8
    }

    #[test]
    fn test413() {
        println!(
            "{}",
            custom_sort_string("cba".to_string(), "abcd".to_string())
        ); // "cbad"
        println!(
            "{}",
            custom_sort_string("cbafg".to_string(), "abcd".to_string())
        ); // "cbad"
    }
}
